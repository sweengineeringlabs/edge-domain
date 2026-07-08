# ADR-045: `edge-llm-runtime` — Standalone LLM Composition Root

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-043 (LLM Complete), ADR-044 (Observability↔LLM Integration), [edge ADR-033](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-033-llm-provider.md) (LLM Provider), [edge ADR-034](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-034-llm-prompt.md) (LLM Prompt), [edge ADR-035](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-035-llm-reasoning.md) (LLM Reasoning), [edge ADR-036](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-036-llm-tools.md) (LLM Tool Governance — proposed, unbuilt), [edge ADR-042](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-042-llmprovider-reshape-to-edge-plugin.md) (llmprovider→plugin reshape), [edge ADR-047](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-047-edge-runtime-primitive-family.md) (`edge-runtime-*` primitive family)
**GitHub Issues:** TBD — new repo creation. Note: this ADR makes `sweengineeringlabs/edge-domain#358` (the `server/scm` `provider_svc.rs` stub) **non-blocking, not resolved** — see Alternatives Considered.

---

## Context

Five LLM domain-primitive crates exist (`edge-llm-provider`, `edge-llm-prompt`, `edge-llm-reasoning`, `edge-llm-complete`, `edge-llm-agent`), each arch-audit-compliant as **libraries**: `api/` contracts, `core/` implementations, `saf/` re-exports. None of them is reachable from a running process today.

A 2026-07-08 audit of the composition-root landscape found **two** parallel assembler crates, not one, with very different maturity:

- **`swe-edge-runtime-server`** (`server/scm`) — has **zero real consumers** anywhere in the monorepo (only its own internal `examples/`). Its `http_route()`/`grpc_route()` registers a `Handler` via `edge_domain::{Domain, Handler}` (`edge-domain-handler`'s `InProcessHandlerRegistry`). The one place meant to register an LLM handler here, `server/scm/main/src/saf/provider_svc.rs`, is six `TODO(#358)` comments and no code. `edge-llm-provider` is not even a dependency of this crate.
- **`swe-edge-bootstrap`** (`edge/scm/bootstrap`) — is the crate that's actually used. `scm/examples/hello-edge` and `scm/examples/hello-llmagents` both depend on it, and `hello_edge.rs` is the **only place in the whole monorepo where an LLM crate has ever run end-to-end**: it registers `edge-llm-provider`'s `StdProviderFactory::default_provider_handler(...)`, dispatches it through a `Job`/`Router`/registry chain, and serves a real HTTP request. Its actual builder API, `RuntimeBuilder::http_route()` (`scm/bootstrap/main/src/api/runtime/types/runtime_builder.rs:83-84`), backs its registry with **`edge_dispatch::HandlerRegistryImpl`** — i.e. `edge-dispatcher` (imported under its Cargo alias `edge_dispatch`, which is why an earlier pass of this audit missed it and wrongly called the crate unused). `edge-dispatcher`'s decorators (`OptionalHandler`, `TimeoutHandler`, `FallbackHandler`, `CacheAsideHandler`) are likewise real, plain `Handler`-in/`Handler`-out wrappers, independent of its `Pipeline` machinery, and already exercised inside `swe-edge-bootstrap` (e.g. `feature_registry_ext.rs`).

So the proven path — the one that has already wired an LLM handler through ingress → registry → domain and gotten a real HTTP response back — is `swe-edge-bootstrap` + `edge-dispatcher`, not `server/scm`. Treating `server/scm` as "the" composition root and edge-dispatcher as dead weight (an earlier draft of this ADR did exactly that) was backwards: it checked the orphaned crate and generalized from it.

That said, `swe-edge-bootstrap` has no consumers of its own outside two throwaway examples either — it has never been packaged as a standalone, deployable, LLM-specific application. Making LLM's first real deployment depend on `server/scm` finishing its unrelated, consumer-less build-out is still coupling we don't want.

## Decision

Create a new, standalone repository, **`edge-llm-runtime`**, that **depends on `swe-edge-bootstrap`** as its composition-root library (not `server/scm`), and registers each LLM crate's `Default*Handler` through `RuntimeBuilder::http_route()`/`grpc_route()` — the same call path `hello_edge.rs` already proved works. This is a standalone repo with its own release cadence, deployable independently of both `server/scm` and the `edge/scm` example workspace — it borrows `swe-edge-bootstrap` and the LLM crates as ordinary library dependencies, the same way `hello-edge`/`hello-llmagents` already do, just as a maintained application instead of a prototype.

### Shape

`edge-llm-runtime` is an **application**, not a domain-primitive crate. It borrows existing, already-proven infrastructure as library dependencies; it does not define new domain contracts of its own:

- `swe-edge-bootstrap::RuntimeBuilder` — the assembler. `.http_route(handler)`/`.grpc_route(handler)` registers each LLM `Arc<dyn Handler<Req,Resp>>`; internally backed by `edge-dispatcher`'s `HandlerRegistryImpl` (real, proven, not experimental — see Context).
- `edge-dispatcher`'s decorators (`TimeoutHandler` in particular) — available for free once `swe-edge-bootstrap` is a dependency. Wrapping the provider handler in a deadline before registration is a natural fit given LLM completion calls can hang; left as an explicit follow-up rather than bundled into the first version, to keep this ADR's first cut minimal.
- `edge-domain-observer::ObserverContext`, `edge-security-runtime::SecurityContext` — required fields of `HandlerContext`, threaded through real (not `Noop`) instances so spans/metrics from ADR-044's seams are actually populated.
- HTTP transport binding via `swe-edge-ingress-http::AxumHttpServer`, the same server `swe-edge-bootstrap`'s own `Runtime` type already wraps.

```
HTTP request
  └─► swe-edge-bootstrap::Runtime / RuntimeBuilder (AxumHttpServer)
        └─► edge_dispatch::HandlerRegistryImpl.get(id)
              └─► Handler::execute(req, HandlerContext{ observer, security, .. })
                    └─► edge-llm-provider / -prompt / -reasoning / -agent (real domain logic)
        └─► HTTP response
```

Per-capability registration:

| Crate | `Default*Handler` | `saf/` factory | Registration-ready today? |
|---|---|---|---|
| `edge-llm-provider` | `DefaultProviderHandler` | `provider_handler_svc::default_provider_handler()` — exists | **Yes** |
| `edge-llm-prompt` | `DefaultPromptHandler` | none yet | No — needs one `saf/prompt_handler_svc.rs` file, mirroring provider's |
| `edge-llm-reasoning` | `DefaultReasoningHandler` | none yet | No — same gap |
| `edge-llm-agent` | `DefaultAgentHandler` | none yet | No — same gap, and its `execute` is still a dispatch stub per ADR-032 |
| `edge-llm-complete` | none | — | N/A — SPI-level port `provider` consumes internally, not ingress-facing |

`edge-llm-provider` is registered first because it needs no prerequisite work — it's the exact crate `hello_edge.rs` already proved out. Prompt/Reasoning/Agent are added as their `saf/` factories are written — mechanical, one file each, not a design question.

### Scope of "end-to-end" for the first version

Today the only `Completer` implementations anywhere in the LLM crates are `EchoCompleter`/`NoopCompleter`/`EchoProviderCompleter` — no real vendor backend exists (ADR-042's plugin was never built). `edge-llm-runtime`'s first version proves the **transport plumbing** — HTTP in → real domain `Handler`/`Provider` logic → HTTP out — using the existing echo backend. It will be reported and documented as exactly that: plumbing proven, vendor backend still open. Wiring a real vendor `Completer` is separate follow-on work, not silently assumed by standing this repo up.

## What this ADR explicitly does NOT solve

Carried over from the 2026-07-08 landscape audit — none of these are fixed by having a composition root, and none should be implied as "handled" once `edge-llm-runtime` exists:

- No real vendor `Completer` (still echo/noop only)
- `edge-llm-reasoning`'s "multi-strategy reasoning" is one linear loop cloned N times, not distinct pattern behavior
- Tool governance (ADR-036) — no capability/risk/policy gate exists; any registered skill runs unrestricted
- No RAG/embeddings/vector retrieval, guardrails/content moderation, cost/usage tracking, eval harness, or real multimodal input path anywhere in `edge/`
- No context-window overflow prevention (`TokenCounter` and `ModelInfo.context_window` are never joined)
- Retry/backoff (`ExecutionError::is_retryable/retry_after`) remains inert — no retry loop consumes it (though `edge-dispatcher`'s `TimeoutHandler` is now a straightforward addition once available — see Shape)

## Consequences

**What this enables**
- A real, runnable, testable path for LLM: HTTP request → real domain logic → HTTP response, built on the one path already proven to work (`hello_edge.rs`'s pattern), not a fresh bespoke assembly.
- LLM crates can ship and be exercised on their own release cadence, independent of `server/scm`'s unrelated, consumer-less build-out.
- `edge-domain#358` stops being a blocker for "can LLM be called at all" — it becomes optional future work (registering LLM in `server/scm` too, for parity, once that crate has real consumers), not a prerequisite.
- A genuine integration-test surface: for the first time, `Provider::complete()` and friends can be exercised over real HTTP, not just unit-tested in isolation.

**What this requires**
- New repo scaffold (`sweengineeringlabs/edge-llm-runtime`), depending on `swe-edge-bootstrap`, `edge-llm-provider` (+ prompt/reasoning/agent once ready), `edge-domain-observer`, `edge-security-runtime`.
- One `saf/<theme>_handler_svc.rs` file each for prompt, reasoning, and agent, mirroring `provider_handler_svc.rs` — small, mechanical, not a design change.
- A naming decision — `edge-llm-runtime` is a placeholder name in this ADR; `edge-llm-server`/`edge-llm-app` are equally reasonable and should be settled before repo creation, not blocked on it.
- No changes to `edge-domain-handler`, `edge-domain-observer`, `edge-dispatcher`, `swe-edge-bootstrap`, or any existing LLM crate's `api/`.

## Alternatives Considered

**Wire LLM into `swe-edge-runtime-server` by finishing `provider_svc.rs` (issue #358)**
Rejected for now, not forever. `server/scm` has no real consumers today and its own composition path (`edge_domain::InProcessHandlerRegistry`) is unproven for LLM specifically — nothing has ever registered an LLM handler through it. Making LLM's first live wiring depend on finishing someone else's consumer-less, in-progress crate was the coupling this ADR avoids. Once `server/scm` gains real adoption, the `saf/` factories built here are directly reusable there too, since both consume the same `Handler` port.

**Hand-roll `Job`/`Router`/`Registry` from scratch, bypassing `swe-edge-bootstrap` entirely**
Rejected. This is what `hello_edge.rs` itself does (it constructs `Domain::new_handler_registry()` and a manual `ProviderJob`/`ProviderRouter` rather than calling `RuntimeBuilder::http_route()`). It works, but duplicates assembly logic `swe-edge-bootstrap`'s `RuntimeBuilder` already provides, tested, with decorator support available. No reason to re-derive it for a maintained app.

**Promote `hello_edge.rs` in place to a real app**
Rejected. It lives inside `edge/scm`'s example workspace; it is example code by convention (not versioned, not tested as a release artifact) and not the right home for a maintained, deployable composition root — but its *pattern* (register `edge-llm-provider`'s handler via the bootstrap `RuntimeBuilder`) is exactly what `edge-llm-runtime` promotes to first-class.

## Tracking

- New repo: `sweengineeringlabs/edge-llm-runtime` (name TBD — see Consequences)
- Prerequisite (parallelizable, non-blocking): `saf/<theme>_handler_svc.rs` for `edge-llm-prompt`, `edge-llm-reasoning`, `edge-llm-agent`
- `sweengineeringlabs/edge-domain#358` — re-scope as "generic composition-root parity," not a blocker for this ADR
- Real vendor `Completer` — separate ADR/issue, explicitly out of scope here
- Follow-up (not blocking first version): wrap the registered provider handler in `edge-dispatcher`'s `TimeoutHandler`
