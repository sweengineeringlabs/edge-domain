# ADR-053: `edge-llm-guardrails` — Content Moderation via a Completer-Wrapping Policy Decorator

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-042 (llmprovider→plugin reshape, plugin-boundary rule), ADR-046 (edge-llm-tools Governance — same Policy-reuse pattern), ADR-048 (Real Vendor Completer — first real backend this decorator will wrap)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit found: **zero edge-side content moderation or guardrail code exists anywhere in `edge/`.** Confirmed directly:

- `domain/scm/domain/llm/provider/main/src/api/provider/errors/execution_error.rs:70-71` declares `ExecutionError::ContentFiltered(String)`.
- `domain/scm/domain/llm/provider/main/src/core/provider/execution/execution_error.rs:100` maps it: `edge_llm_complete::CompleteError::ContentFiltered(m) => Self::ContentFiltered(m)` — a pure passthrough.
- `domain/scm/domain/llm/complete/main/src/api/complete/errors/complete_error.rs:40` declares the upstream `CompleteError::ContentFiltered(String)` this is mapped from.
- A repo-wide grep of `domain/scm/domain/llm` for `ContentFiltered` finds exactly two *construction* sites, both mechanical `From`/reverse-`From` mappings between these two enums (`execution_error.rs:100`, `provider_completer.rs:36`). **No code anywhere constructs `ContentFiltered` from first principles.** It is purely reactive: a value that would only ever appear if some `Completer` implementation decided, on its own, that a request or response should be filtered.
- Today the only `Completer` implementations in the workspace are `EchoCompleter`, `NoopCompleter`, and `EchoProviderCompleter` (confirmed via `grep -l "impl Completer for"` — 3 hits, none does any content inspection). None of them ever constructs `ContentFiltered`. ADR-048 (proposed same day) is what stands up the first real vendor `Completer` (`AnthropicCompleter`); even once that ships, `ContentFiltered` would only fire if Anthropic's own moderation rejects a request — still zero edge-side policy.

So the honest state is: `ExecutionError::ContentFiltered` is a well-formed, already-plumbed *failure surface* with nothing behind it. There is no keyword filter, no PII check, no jailbreak heuristic, no policy of any kind applied to a prompt before it leaves the perimeter or to a completion before it reaches a caller.

### The seam that actually sees every completion call

Provider-level or Handler-level enforcement looks like the obvious place to add this (mirroring ADR-046's `GovernedHandler`), but tracing actual call sites shows it would miss real traffic:

- `domain/scm/domain/llm/complete/main/src/core/complete/bounded_tool_call_loop.rs:43` — `BoundedToolCallLoop::run()` calls `self.completer.complete(CompleteRequest { request: &current })` **directly** on its injected `Arc<dyn Completer>`, once per turn of a multi-turn tool-calling loop. This never goes through `edge_llm_provider::Provider`, `StdProvider::complete()`, or any `Handler` — it is entirely internal to `edge-llm-complete`.
- `domain/scm/domain/llm/provider/main/src/core/provider/std/std_provider.rs:134-150` (`StdProvider::complete`) is a *second*, separate call site that also just delegates to `self.completer.complete(...)`.
- `domain/scm/domain/llm/provider/main/src/core/provider/default_provider_handler.rs` (`DefaultProviderHandler`) wraps `ExecutionModel`, not `Completer`/`Provider` directly, and its `Request`/`Response` types (`String`, `ExecutionStepResult`) require unwrapping before any raw prompt/response text is visible.

The one type both call sites share is `Arc<dyn edge_llm_complete::Completer>` itself — `CompleteRequest<'_> { request: &CompletionRequest }` in, `CompletionResponse { content: Option<String>, .. }` out, both already plain, vendor-agnostic text. A decorator implementing `Completer` and wrapping the same `Arc<dyn Completer>` that is constructed once and handed to *both* `StdProvider::new()` and `BoundedToolCallLoop` sees every completion call in the system uniformly, regardless of which higher-level path issued it.

## Decision

Build a small new crate, **`edge-llm-guardrails`**, following ADR-046's discipline exactly: no new governance vocabulary invented where a port already exists.

### Kept as new (genuinely novel, no existing analog)

- **`GuardrailCheckRequest<'a>`** — `{ text: &'a str, phase: GuardrailPhase }`. The one new input shape a content check needs: the text to inspect, and which side of the call it came from.
- **`GuardrailPhase`** — `{ PreCall, PostCall }`. Lets an individual policy restrict itself to one direction (e.g. an outbound-secret-leak check only makes sense `PreCall`) while most policies (a denylist) apply to both.
- **`DenylistPolicy`** — the baseline, pluggable keyword/regex denylist. Real ML-based moderation (a vendor classifier or moderation endpoint) is explicitly a plugin-level concern per ADR-042's boundary — out of scope for this contract; a plugin providing one just plugs into the same `CompositePolicy` as another `Policy` impl, no contract change needed later.
- **`GuardrailedCompleter`** — the decorator (see Enforcement mechanism).

### Reused, not reinvented

- **No new `GovernancePolicy`/moderation trait.** A guardrail check is just `edge_domain_policy::Policy<Input = GuardrailCheckRequest>`. `DenylistPolicy` (and any future ML-moderation policy) implements it. Multiple policies compose via `CompositePolicy::new().with(Box::new(global_denylist)).with(Box::new(tenant_denylist))` — AND semantics, first violation short-circuits, exactly the ADR-046 pattern, using the port that already exists.
- **No new error type.** `ExecutionError::ContentFiltered` and `edge_llm_complete::CompleteError::ContentFiltered` already exist end-to-end (see Context) and are exactly the right shape (`String` reason). On a `PolicyError` from `CompositePolicy::evaluate`, `GuardrailedCompleter` constructs `CompleteError::ContentFiltered(policy_error.reason)` directly — no `From` impl needed (both types are foreign to this crate; the mapping is an ordinary expression at the one call site, not a trait). `ExecutionError::from(CompleteError)` already forwards it unchanged (`execution_error.rs:100`), so `StdProvider::complete()` requires **zero code changes** to surface a guardrail denial as `ExecutionError::ContentFiltered` to every existing caller.
- **No new text-extraction logic for prompts.** `edge_llm_complete::ContentFlattener::flatten(FlattenRequest { content: &MessageContent }) -> FlattenResponse { text: String }` already exists (`complete/main/src/api/complete/traits/content_flattener.rs`) precisely to turn a `Message`'s `MessageContent` (`Text`/`Parts`/`Empty`) into plain text. `GuardrailedCompleter` takes an injected `Arc<dyn ContentFlattener>` and flattens each `Message.content` in `CompletionRequest.messages`, concatenating the results, rather than hand-rolling a second flattener.
- **Post-call extraction needs no flattening at all** — `CompletionResponse.content` is already `Option<String>`.
- **Observability reuses `ObserverContext`**, the same seam ADR-044/046 already use: a span attribute (`guardrail.decision`) plus a counter (`guardrail_checked_total{phase=..,allowed=..}`) recorded around each policy evaluation, via an `Arc<dyn ObserverContext>` injected at construction — the identical pattern `StdProvider` itself already uses for its own `observer` field.

### Enforcement points: both pre-call and post-call (for blocking `complete`); pre-call only for `complete_stream`, by design

Both directions are implemented in this ADR, not just one, because they are the same mechanism (one `CompositePolicy<GuardrailCheckRequest>`, evaluated twice) guarding two distinct, real risks:

- **Pre-call** (`GuardrailPhase::PreCall`, checked against the flattened, concatenated `CompletionRequest.messages` before `self.inner.complete(...)` is invoked): stops a banned/injected payload from ever leaving the perimeter to a vendor, and — just as importantly — avoids paying for a vendor call that would only be rejected anyway.
- **Post-call** (`GuardrailPhase::PostCall`, checked against `CompletionResponse.content` after the inner completer returns, before the response is handed back to *any* caller): this is the case `ExecutionError::ContentFiltered` was named for — a generated response containing content that must not reach the caller, independent of whether the vendor's own moderation caught it.

Deferring either one would leave a caller with no protection in that direction from day one for no implementation savings — the code path is identical.

`complete_stream` gets the pre-call check on the same terms (it shares the same `CompletionRequest`). **Post-call streaming enforcement is explicitly out of scope here**: per ADR-048, every `Completer` today (`EchoCompleter`, `NoopCompleter`, `EchoProviderCompleter`) implements `complete_stream` as a single `stream::once` chunk, so buffering the "stream" to run a post-call check would be nearly free right now — but hard-coding that assumption would silently break once ADR-048's real incremental SSE streaming ships, at which point buffering the full stream before checking defeats the purpose of streaming (adds full-response latency back). This ADR does not solve incremental (per-chunk or sliding-window) moderation of a genuinely streamed response; it is named as follow-up work, not silently assumed.

### Enforcement mechanism: a `Completer`-wrapping decorator, not a `Handler` decorator or an inline step in `StdProvider::complete()`

`GuardrailedCompleter` implements `edge_llm_complete::Completer` and wraps an inner `Arc<dyn Completer>`:

```rust
pub(crate) struct GuardrailedCompleter {
    inner: Arc<dyn Completer>,
    policy: Arc<CompositePolicy<GuardrailCheckRequest>>,
    flattener: Arc<dyn ContentFlattener>,
    observer: Arc<dyn ObserverContext>,
}

#[async_trait]
impl Completer for GuardrailedCompleter {
    async fn complete(&self, req: CompleteRequest<'_>) -> Result<CompletionResponse, CompleteError> {
        let prompt_text = flatten_messages(&self.flattener, &req.request.messages)?; // via ContentFlattener
        self.check(&prompt_text, GuardrailPhase::PreCall)?;

        let response = self.inner.complete(req).await?;

        if let Some(content) = &response.content {
            self.check(content, GuardrailPhase::PostCall)?;
        }
        Ok(response)
    }
    // complete_stream: same pre-call check, then delegates unchanged (see above)
    // supported_models / model_info / list_models / supports / is_model_available / health_check:
    //   pass straight through to `inner` — guardrails are a content concern only
}
```

Justification, ranked by why the alternatives were rejected (see Alternatives Considered for detail):

1. **Coverage** — `Arc<dyn Completer>` is the one type both real call sites (`StdProvider::complete()` and `BoundedToolCallLoop::run()`, see Context) already share. Wrapping it once, at construction, protects both uniformly. A `Handler` decorator (ADR-046's `GovernedHandler` shape) would only guard whatever gets registered as a `Handler` — it would never see `BoundedToolCallLoop`'s internal per-turn calls, since those never pass through a `Handler` at all.
2. **Text is already plain at this seam** — `CompletionRequest.messages` / `CompletionResponse.content` are the first (and, on the request side, only) point where prompt/response content exists as inspectable text without unwrapping a `CompletionInput` or an `ExecutionStepResult`.
3. **Zero changes to `Provider`, `StdProvider`, `Handler`, or `Completer`'s own trait** — exactly ADR-046's "compose at construction, don't touch the domain trait" discipline. `StdProvider::new(config, model, completer, observer)` already takes `completer: Arc<dyn Completer>` as an ordinary constructor argument; the composition root (or a `saf/` factory) wraps it in `GuardrailedCompleter` *before* passing it in. `BoundedToolCallLoop`'s constructor takes the same `Arc<dyn Completer>` and is wrapped identically.
4. **Applies uniformly to every future vendor** — per ADR-042/048's plugin-boundary rule, `AnthropicCompleter` (and any later vendor plugin) is just another `Completer` impl; wrapping happens once, above any vendor, so no individual plugin needs to remember to implement moderation itself.

The rejected inline-in-`StdProvider::complete()` option would require threading a `CompositePolicy` field into `StdProvider` itself (a new required constructor argument on a crate that is otherwise complete, arch-clean, and has no policy dependency today) and would still miss `BoundedToolCallLoop` entirely, since that loop never touches `StdProvider`.

### Shape / workspace layout

```
domain/scm/domain/llm/guardrails/          (edge-llm-guardrails)
├── api/
│   └── types/{guardrail_check_request.rs, guardrail_phase.rs}
├── core/
│   ├── denylist_policy.rs          (impl Policy<Input = GuardrailCheckRequest>; keyword + regex baseline)
│   └── guardrailed_completer.rs    (the decorator, impl Completer)
└── saf/
    └── guardrailed_completer_svc.rs   (guardrailed_completer(inner, policy, flattener, observer) -> impl Completer)
```

Depends on: `edge-llm-complete` (`Completer`, `CompleteError`, `CompletionRequest`/`CompletionResponse`, `Message`/`MessageContent`, `ContentFlattener`/`FlattenRequest`/`FlattenResponse`), `edge-domain-policy` (`Policy`, `CompositePolicy`, `PolicyError`), `edge-domain-observer` (`ObserverContext`). No new dependency on `edge-security-runtime` — see limitation below.

## What this ADR explicitly does NOT solve

- **Tenant-aware / per-caller policy variation.** `Completer::complete(req: CompleteRequest<'_>)` carries no `SecurityContext` — unlike `Handler::execute`, this seam has no caller identity available. `GuardrailCheckRequest` therefore cannot vary its check by tenant or principal; `CompositePolicy` here is effectively global per `GuardrailedCompleter` instance. If per-tenant denylists become a real requirement, the fix is a *second*, `Handler`-level `GovernedHandler`-style decorator (ADR-046 shape, with `ctx.security` available) layered on top — not a change to this one. Named explicitly rather than silently assumed away.
- **Real ML-based moderation** (a vendor classifier/moderation endpoint, embeddings-based similarity checks, etc.) — per ADR-042's boundary, that is a plugin providing another `Policy` impl, not part of this contract.
- **Incremental (per-chunk) post-call moderation of a genuinely streamed response** — see Enforcement points above; only pre-call is covered for `complete_stream` in this ADR.
- **PII detection/redaction as a distinct concern from a denylist** — `DenylistPolicy` is keyword/regex matching only; a dedicated PII-scanning `Policy` impl is a natural follow-on `Policy`, not built here.
- **Retrofitting guardrails onto any already-constructed `Completer` automatically** — exactly ADR-046's admitted limitation for `GovernedHandler`: each composition root must explicitly wrap its `Arc<dyn Completer>` in `GuardrailedCompleter` before injecting it into `StdProvider`/`BoundedToolCallLoop`; an unwrapped `Completer` is still possible and is not flagged by tooling.
- **Registering this behind `edge-llm-runtime` (ADR-045) or wiring `AnthropicCompleter` (ADR-048) through it** — this ADR produces the decorator and baseline policy; wiring it into a live composition root is separate follow-on work, the same carve-out ADR-045/046/048 already use for their own scopes.

## Consequences

**What this enables**
- The first edge-side content check anywhere in the LLM stack — closing the single most literal reading of the 2026-07-08 audit's finding ("no guardrails/content-moderation exist anywhere").
- `ExecutionError::ContentFiltered`/`CompleteError::ContentFiltered` go from a dead, unreachable enum arm to an actually-constructible, edge-owned failure — with **zero changes** to either error type or to `StdProvider`'s `complete()` method.
- Coverage of every real completion call site found in the current codebase (`StdProvider::complete()` *and* `BoundedToolCallLoop::run()`), not just the one that happens to be registered as a `Handler`.
- A pluggable seam for real moderation later (vendor classifier, PII scanner) that composes into the same `CompositePolicy` with zero contract change, mirroring ADR-046's `CompositePolicy` reuse story.

**What this requires**
- New crate `edge-llm-guardrails` under `domain/scm/domain/llm/guardrails/`.
- Every composition root that constructs an `Arc<dyn Completer>` today (`swe-edge-bootstrap`'s `hello_edge.rs` pattern per ADR-045, any future `edge-llm-runtime` wiring, `BoundedToolCallLoop` construction sites) must be updated to wrap it in `GuardrailedCompleter` — a real, per-call-site wiring task, not automatic.
- A baseline denylist to actually configure (`DenylistPolicy`'s keyword/regex list) — shipping the crate with an empty list is a no-op; someone must populate a real starter list before this closes the gap in practice.
- No changes to `edge-llm-complete`'s `api/` (`Completer`, `CompleteError`, `ContentFlattener` all already sufficient), no changes to `edge-llm-provider`'s `Provider`/`StdProvider`, no changes to `edge-domain-policy`, `edge-domain-observer`.

## Alternatives Considered

**A `Handler`-wrapping decorator (`GuardedHandler`, mirroring ADR-046's `GovernedHandler` exactly)**
Rejected for the primary mechanism. Structurally the same shape as `GovernedHandler`, but `Handler::execute`'s `Request`/`Response` are handler-specific (`DefaultProviderHandler` uses `String`/`ExecutionStepResult`, wrapping `ExecutionModel`, not `Completer`) and — decisively — `BoundedToolCallLoop::run()` never goes through a `Handler` at all (see Context). A `Handler` decorator would leave every completion call inside a tool-calling loop completely unguarded. Not rejected as *never useful*: it remains the right mechanism for the tenant-aware follow-up named above, where `ctx.security` is required and only available at that layer.

**An inline step inside `StdProvider::complete()`**
Rejected. `std_provider.rs:134-150` would need a new `CompositePolicy` field threaded through `StdProvider::new()`'s constructor — a required-argument change to an already arch-clean, 183/183 crate — and would still only cover `StdProvider`'s one call path, missing `BoundedToolCallLoop` entirely. Wrapping the shared `Arc<dyn Completer>` both consumers already take achieves full coverage with no signature change to either.

**A bespoke `GuardrailPolicy`/`ModerationContext`/`ModerationError` design (ADR-036-style, before its own later correction)**
Rejected outright, following ADR-046's precedent directly: `edge_domain_policy::Policy<Input=T>` + `CompositePolicy` already provide exactly this shape (named rule, `evaluate` → `Result<(), PolicyError>`, AND-composable), and `ExecutionError`/`CompleteError::ContentFiltered` already provide the failure surface. Inventing parallel types would duplicate both for no behavioral gain, plus a duplicated set of arch-audit obligations (own error-naming rule, own test scenarios) ADR-046 already flagged as the wrong tradeoff.

## Tracking

- New crate: `edge-llm-guardrails` (`domain/scm/domain/llm/guardrails/`)
- Follow-up (not blocking): wire `GuardrailedCompleter` into `swe-edge-bootstrap`'s `hello_edge.rs` pattern (ADR-045) and any future `edge-llm-runtime` composition root
- Follow-up (not blocking): wire `GuardrailedCompleter` around `AnthropicCompleter` once ADR-048 ships
- Follow-up (separate ADR, named as an explicit gap above): tenant-aware policy variation via a `SecurityContext`-carrying `Handler`-level decorator, layered on top of this one
- Follow-up (separate ADR, named as an explicit gap above): incremental post-call moderation once ADR-048's real streaming lands
- Not blocking this ADR: real ML-based moderation policy, dedicated PII-scanning policy — both plug into the same `CompositePolicy` later with no contract change
