# ADR-057: LLM End-to-End Dataflow Trace — Composition Order Across ADR-045..056

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-045 (Composition Root), ADR-046 (Tool Governance), ADR-047 (MCP), ADR-048 (Real Vendor Completer), ADR-049 (Reasoning Honesty), ADR-050 (Context-Window Guard), ADR-051 (Retry/Backoff), ADR-052 (Retrieval), ADR-053 (Guardrails), ADR-054 (Cost/Usage Tracking), ADR-055 (Eval Harness), ADR-056 (Multimodal Input), ADR-071 (Persistent Usage Ledger + SpendLimitPolicy — adds a step to this trace, see below)
**GitHub Issues:** TBD

**Amendment (post-review, via ADR-071):** `SpendLimitPolicy` (ADR-071) adds a new step to the resolved trace below, immediately after the context-window check and before the guardrail's `PreCall` phase — deny an over-budget request before paying for the call, for the same reason the context-window check runs early. ADR-071 named this as a required follow-up to this ADR rather than silently diverging from it; folded in below.

---

## Context

ADR-045 through ADR-056 were drafted independently and in parallel — each agent read only ADR-045/046 as a style template, not each other's decisions. Each ADR has a **local** dataflow diagram for its own integration point. None traces a single request through all of them together, in the order it would actually execute. Reconstructing that trace surfaces one structural gap and five unresolved ordering/consistency questions.

### The structural gap: two disconnected completion paths

`DefaultProviderHandler` (`provider/main/src/core/provider/default_provider_handler.rs:40-70`) is the `Handler` that ADR-045 registers via `StdProviderFactory::default_provider_handler(...)` — the one `hello_edge.rs` proves reachable from a live HTTP request today. Its `execute` method does exactly one piece of domain work:

```rust
let result = self.model.execute_step(StepExecutionRequest {
    agent_id: "", goal: &req.req, context: "", available_tools: Vec::new(),
}).await...
```

`self.model: Arc<dyn ExecutionModel>` — `ExecutionModel::execute_step` (`provider/main/src/api/provider/traits/execution_model.rs:16-19`) is a **different trait** from `Provider::complete()` (`provider/main/src/api/provider/traits/provider.rs:74-77`), with no code anywhere converting between them. `EchoExecutionModel` (the only `ExecutionModel` impl in the repo) does not call `Provider`/`Completer` internally — it's an independent echo, same as `EchoCompleter`/`NoopCompleter`.

Meanwhile, **every one of ADR-050 (context-window guard), ADR-053 (guardrails), ADR-054 (usage tracking), and ADR-056 (multimodal)** hooks into `Provider::complete()` or wraps the `Arc<dyn Completer>` that `StdProvider` holds. All four are correctly designed against the right port — `Provider::complete()` is the real, structured completion path (ADR-043-aligned, wired to a real `Completer` call in this session's earlier `78a4d8c` commit) — but **none of that work is reachable from a live HTTP request today**, because the registered `Handler` never calls `Provider::complete()` at all. It calls `ExecutionModel::execute_step()`, a parallel, unbridged path.

This is not a defect introduced by any of the four ADRs — each was correctly scoped against `Provider`/`Completer` as instructed. It's a pre-existing fork this dataflow trace is the first artifact to surface.

### Five composition-order and consistency questions left unresolved

1. **`GovernedHandler` (ADR-046) vs. `RetryHandler` (ADR-051)** — both wrap the same `Handler`. No ADR states which wraps which.
2. **Retrieval (ADR-052) vs. context-window guard (ADR-050)** — retrieval injects content into `ContextManager` before rendering; the context-window check must see the *post-retrieval* token count or it validates a stale, smaller estimate.
3. **Guardrail pre-call phase (ADR-053) vs. context-window guard (ADR-050)** — both gate the outgoing call, at different layers (`Completer`-wrapping vs. a `CompositePolicy` field on `StdProvider`). Relative order was never decided.
4. **Usage recording (ADR-054) vs. retry (ADR-051)** — `UsageRecorder::record` is called inside `StdProvider::complete()`, once per invocation. If `RetryHandler` retries at the `Handler` layer above, does a failed-then-retried call record usage once or per attempt?
5. **`ObserverContext` instance consistency across independently-constructed seams.** `StdProvider` (`observer` field), `GuardrailedCompleter` (ADR-053, `observer` field), `DefaultUsageRecorder` (ADR-054, holds its own `Arc<dyn ObserverContext>`), and `GovernedHandler` (ADR-046, emits through `ctx.observer`) each obtain an `ObserverContext` independently — three by their own constructor parameter, one via `HandlerContext`. Nothing states they must all be the *same* `Arc<dyn ObserverContext>` instance. Verified: `ObserverContext`'s real shape (`domain-observer/main/src/api/observe/traits/observer_context.rs:13-19`) has no identity/equality concept a caller could use to detect divergence at runtime — if a composition root constructs these four with different instances (e.g. two separate `noop_observer_context()` calls), a guardrail denial's span and the request's own tracer span land in unrelated trace contexts and never correlate, silently. None of ADR-046/053/054, nor this ADR until now, named the invariant.

## Decision

### Resolve the structural gap: bridge `ExecutionModel` to `Provider`

Build `StdExecutionModel: ExecutionModel`, implemented by converting `StepExecutionRequest` into a `ProviderCompleteRequest`/`CompletionInput` and delegating to an injected `Arc<dyn Provider>`, converting `ProviderCompletionResponse` back into `StepExecutionResponse`. This is the same shape of gap as ADR-048 (no real `Completer`) and ADR-042 (no real vendor plugin) one layer up: `ExecutionModel` has no real implementation today, only an echo stub, for the same reason `Completer` didn't.

This is the recommended fix over the alternative (changing `DefaultProviderHandler` to call `Provider::complete()` directly, retiring `ExecutionModel`): `ExecutionModel::execute_step`'s flat, string-goal shape (`agent_id`, `goal`, `context`, `available_tools`) is what `edge-llm-reasoning`'s step-loop already programs against (per ADR-049's `linear_reasoning.rs`/`bounded_tool_call_loop.rs` research) — retiring the trait would ripple into the reasoning crate for no benefit. Bridging preserves both ports' purposes and makes the fix local to `provider/core/`.

Once this bridge exists, the chain is:
```
DefaultProviderHandler::execute
  → ExecutionModel::execute_step   (StdExecutionModel, NEW)
      → Provider::complete()       ← ADR-050/053/054/056 all become reachable here
          → Completer::complete()  (ADR-048's real vendor Completer, once it ships)
```

Until `StdExecutionModel` exists, ADR-050/053/054/056/071 remain correctly designed but **dormant from an HTTP caller's perspective** — exercisable only through direct unit/integration tests that construct a `StdProvider` and call `.complete()` themselves, not through `edge-llm-runtime`'s registered ingress. This ADR does not ask any of those five ADRs to change; it adds the one missing piece that makes them live.

### Resolve the five ordering/consistency questions

1. **`GovernedHandler` outside `RetryHandler`.** Deny-fast: a capability/risk denial must never be retried. Composition: `GovernedHandler::new(RetryHandler::new(inner))`. A denied call never reaches the retry loop; a transient failure on an *allowed* call gets retried.
2. **Retrieval before context-window guard, structurally guaranteed, not just by convention.** Retrieval (ADR-052) composes into `ContextManager` *before* `Prompt::render`/`build_context` runs; the context-window guard (ADR-050) runs later still, inside `StdProvider::complete()`, over the already-rendered `CompletionInput`. Because these three steps already sit in that pipeline order (retrieve → render → build request → guard), no new sequencing code is needed — this ADR's contribution is stating the invariant explicitly so a future change doesn't reorder it silently.
3. **Context-window guard before guardrail pre-call check.** Rationale: the size check is the cheaper failure to detect (integer comparison vs. flattening + policy evaluation over message content) and — more importantly — independent of content, so checking it first avoids paying the guardrail's text-flattening cost (`ContentFlattener::flatten`, per ADR-053) on a request that's going to be rejected for size regardless. The two are designed to live at different layers (`StdProvider`'s `CompositePolicy` field vs. `GuardrailedCompleter` wrapping `Arc<dyn Completer>` — neither exists in the codebase yet; both are `Status: Proposed`) — once built, `StdProvider::complete()` would naturally run its own `CompositePolicy` check before ever calling `self.completer.complete(...)`, which would then be the `GuardrailedCompleter`-wrapped instance, so this ordering falls out of the two ADRs' own designs without new code, provided composition roots wire `GuardrailedCompleter` in at construction (as ADR-053 already requires) rather than at some earlier point.
4. **Usage recorded once per successful call, not per attempt.** `RetryHandler` wraps at the `Handler` layer, `UsageRecorder` records inside `StdProvider::complete()`, one layer below. Each retry attempt is a fresh `StdProvider::complete()` invocation, so a naive reading would record usage per attempt. This ADR specifies the intended behavior explicitly: usage **should** be recorded per attempt (each attempt consumes real vendor tokens whether or not it ultimately succeeds), not just the final one — the alternative (recording only on final success) would silently undercount spend on any request that failed once and succeeded on retry. `UsageRecorder`'s emission already happens inside `StdProvider::complete()`, so per-attempt recording is what the current design already does; this ADR resolves the ambiguity in ADR-054's own text by stating it's intentional, not an oversight to fix.
5. **One shared `Arc<dyn ObserverContext>` per composition root, constructed once and cloned into every seam.** The composition root that builds `StdProvider`, `GuardrailedCompleter`, `DefaultUsageRecorder`, and the `Handler` chain (`GovernedHandler`/`RetryHandler`) must construct exactly one `Arc<dyn ObserverContext>` (real backend or `noop_observer_context()` in tests) and pass clones of that same `Arc` into all four — never four independent instances. This is a wiring discipline, not a code change to any of ADR-046/053/054's own designs: each already accepts `Arc<dyn ObserverContext>` as an ordinary constructor parameter: the fix is entirely in how the composition root calls those constructors.

### The full resolved trace

```
HTTP/gRPC request                                    MCP tool-call (ADR-047, alt. ingress)
  │                                                        │
  ▼                                                        ▼
swe-edge-bootstrap::Runtime (ADR-045)              edge-llm-mcp adapter (ADR-047)
  │  Job → Router → HandlerRegistryImpl.get(id)           │  same registry lookup
  ▼                                                        ▼
GovernedHandler<RetryHandler<DefaultProviderHandler>>  (ADR-046 outside ADR-051, per #1 above)
  │  1. CompositePolicy(capability, risk) — deny fast, no retry on denial
  │  2. bounded retry loop begins (ADR-051)
  ▼
DefaultProviderHandler::execute
  ▼
ExecutionModel::execute_step  →  StdExecutionModel (NEW, this ADR — the missing bridge)
  ▼
Provider::complete()  [provider/core/provider/std/std_provider.rs]
  │  a. retrieval already composed into ContextManager before render (ADR-052 — upstream of this call)
  │  b. ContextWindowPolicy check (ADR-050) — cheap, content-independent, runs first
  │  c. SpendLimitPolicy check (ADR-071) — same CompositePolicy as (b), runs immediately after: deny before paying for an over-budget call, checked per attempt (same reasoning as usage recording, step (e))
  │  d. self.completer.complete(...) — completer is GuardrailedCompleter-wrapped (ADR-053)
  │       i.   GuardrailPhase::PreCall check (flattened prompt text)
  │       ii.  real vendor Completer (ADR-048, once shipped) — Anthropic Messages API
  │       iii. GuardrailPhase::PostCall check (response content)
  │  e. UsageRecorder::record(...) — per attempt, per #4 above (ADR-054)
  ▼
response bubbles back through Provider → ExecutionModel → Handler
  │  on ExecutionError::is_retryable() == true: RetryHandler retries from "bounded retry loop begins" (bounded attempts, per ADR-051)
  ▼
HTTP/gRPC/MCP response

separately, off this path entirely:
edge-llm-eval (ADR-055) — calls Provider/Agent directly, never touches the registry
```

Every box in the trace above that independently takes an `Arc<dyn ObserverContext>` (`GovernedHandler`, `StdProvider`, `GuardrailedCompleter`, `DefaultUsageRecorder`) must receive a clone of the *same* `Arc`, constructed once by the composition root — per #5 above. This is not shown as a separate box because it's not a pipeline stage; it's a constraint on how every other box gets built.

Reasoning-pattern selection (ADR-049) and multimodal content (ADR-056) are not separate boxes in this trace — they're shape decisions made *before* `Provider::complete()` is called (which pattern's step logic assembled the `CompletionInput`, whether a `ContentPart::Image` is present in it) rather than additional pipeline stages.

## What this ADR explicitly does NOT solve

- Does not implement `StdExecutionModel` — names the bridge and its shape, doesn't build it.
- Does not resolve whether `edge-llm-reasoning`'s `BranchingReasoning`/`ReflectiveReasoning` (ADR-049) call `ExecutionModel` or `Provider` directly — that's a detail internal to the reasoning crate's own step logic, out of scope here.
- Does not change any of ADR-045/046/048/049/050/051/052/053/054/055/056's own designs — every ordering/gap resolution above is additive (a bridge, a composition order, an explicit intent statement), not a redesign of any existing decision.
- Does not address MCP's client direction (ADR-047 deferred it) — the trace above shows only the server direction as an alternate ingress.

## Consequences

**What this enables**
- A concrete build order: `StdExecutionModel` (this ADR) must land before ADR-050/053/054/056/071's work is observable from any real request, regardless of which order those five ADRs themselves are implemented in.
- Removes a likely-otherwise-silent integration failure: implementing all of ADR-050/053/054/056/071 correctly in isolation, running their unit tests green, and then discovering in an end-to-end test that none of it fires on a real HTTP request.

**What this requires**
- New `StdExecutionModel` in `provider/core/provider/` (or similar), converting `StepExecutionRequest ⇄ ProviderCompleteRequest`/`StepExecutionResponse ⇄ ProviderCompletionResponse`.
- `DefaultProviderHandler`'s constructor (or its `saf/` factory) updated to construct `StdExecutionModel` wrapping a real `StdProvider`, instead of `EchoExecutionModel`, once a real `Completer` (ADR-048) exists — until then it remains correct to construct it over the echo backend, proving plumbing only, per ADR-045's own stated scope.
- Composition-root wiring code (wherever `GovernedHandler`/`RetryHandler` get constructed) must apply them in the stated order (`Governed(Retry(inner))`), and wherever `Arc<dyn Completer>` is constructed must wrap it in `GuardrailedCompleter` before injecting into `StdProvider`, per ADR-053's own tracking item.

## Alternatives Considered

**Retire `ExecutionModel`, change `DefaultProviderHandler` to call `Provider::complete()` directly**
Rejected as the primary fix. Would require converting `DefaultProviderHandler`'s `Handler<Request = String, Response = ExecutionStepResult>` shape to something `CompletionInput`/`ProviderCompletionResponse`-typed — a breaking `Handler` signature change — and would strand `edge-llm-reasoning`'s existing step-loop code, which programs against `ExecutionModel`'s flat request shape today. Bridging is strictly additive; retiring is a breaking rewrite for the same end state.

**Leave the two paths unbridged and duplicate the cross-cutting concerns (context-window, guardrails, usage) inside `ExecutionModel`/`EchoExecutionModel` directly**
Rejected. Would mean implementing context-window checking, guardrails, and usage tracking twice — once against `Provider`/`Completer` (as ADR-050/053/054 already specify) and once against `ExecutionModel` — for the same underlying concern. The bridge makes one implementation serve both entry points.

## Tracking

- New: `StdExecutionModel` bridging `ExecutionModel` → `Provider::complete()` — blocks ADR-050/053/054/056/071 from being HTTP-observable
- Composition-root wiring note: `GovernedHandler(RetryHandler(inner))` ordering — applies wherever ADR-045/046/051 are actually implemented
- Composition-root wiring note: `SpendLimitPolicy` (ADR-071) joins `StdProvider`'s `CompositePolicy` immediately after `ContextWindowPolicy`, before the `GuardrailedCompleter` handoff (per the amendment above)
- Composition-root wiring note: wrap every constructed `Arc<dyn Completer>` in `GuardrailedCompleter` (ADR-053's own tracking item, reaffirmed here as part of the full trace)
- Composition-root wiring note: construct exactly one `Arc<dyn ObserverContext>` per composition root and clone it into `StdProvider`, `GuardrailedCompleter`, `DefaultUsageRecorder`, and the `GovernedHandler`/`RetryHandler` chain — never independent instances (per #5 above)
- Follow-up: an integration test asserting observer-instance sharing specifically — e.g. a test double `ObserverContext` with an instance-identifiable counter, verifying a single request's guardrail check, usage record, and handler-level span all land on the same instance — would catch a regression here that a purely functional test would not
- Follow-up: once `StdExecutionModel` exists, add an end-to-end integration test exercising the full trace above (HTTP in → guard → retry → bridge → provider → guardrail → completer → usage → response out) against echo backends, mirroring ADR-045's own "prove the plumbing" scope
