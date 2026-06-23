# ADR-044: Observability↔LLM Integration — two injection seams

**Status:** Proposed  
**Date:** 2026-06-19  
**Governing ADR:** [ADR-006](ADR-006-observability-domain-primitive.md) — Observability Domain Primitive  
**Relates to:** [ADR-033](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-033-llm-provider-domain-primitive.md) — LLM Provider Domain Primitive, [ADR-043](ADR-043-llm-complete-domain-primitive.md) — LLM Complete Domain Primitive  
**GitHub Issues:** TBD — ObserveContext trait in edge-domain-observer, TBD — HandlerContext observe seam, TBD — ProviderFactory observe seam

---

## Context

`edge-domain-observer` (ADR-006) and the five LLM crates (`edge-llm-complete`, `edge-llm-provider`, `edge-llm-agent`, `edge-llm-prompt`, `edge-llm-reasoning`) are currently unconnected. No LLM crate imports from `edge-domain-observer`. No handler dispatch path carries observability context.

Two observable failure modes exist today:

1. **Silent agent dispatch.** When a `Skill` executes inside a `HandlerContext`, there is no way for the infrastructure to attach a trace span, emit a log record, or record a latency histogram — the call is invisible to the observability plane.

2. **Invisible provider calls.** When `ProviderFactory::provider(...)` constructs a `Provider` and the provider calls a `Completer`, there is no instrumentation point for token usage, model name, or API round-trip latency. These metrics are the most operationally significant in an LLM-backed system and are currently unrecordable at the domain level.

---

## As-found baseline (audit 2026-06-19)

A full structural audit of the workspace was conducted before writing this ADR. The findings below correct assumptions that would otherwise produce wrong implementation plans.

### HandlerContext is an enum, not a struct

```rust
// domain-handler/main/src/api/handler/types/handler_context.rs
#[derive(Copy, Clone)]
pub enum HandlerContext<'a> {
    Standard {
        security: &'a SecurityContext,
        commands: &'a dyn CommandBus,
    },
}
```

The `observer` field must be added to the `Standard` variant, not to a struct. The `Copy + Clone` derive is preserved because the type holds only references.

### All four LLM handlers ignore HandlerContext

Every concrete handler prefixes the context with `_`:

```rust
async fn execute(&self, goal: String,    _ctx: HandlerContext<'_>) -> ...  // DefaultProviderHandler
async fn execute(&self, input: String,   _ctx: HandlerContext<'_>) -> ...  // DefaultReasoningHandler
async fn execute(&self, ctx: RenderContext, _ctx: HandlerContext<'_>) -> ...  // DefaultPromptHandler
async fn execute(&self, input: String,   _ctx: HandlerContext<'_>) -> ...  // DefaultAgentHandler
```

No security enforcement, no command dispatch, and no observability happens inside any handler body today.

### DefaultAgentHandler is a stub

```rust
// domain/llm/agents/main/src/core/types/default_agent.rs
async fn execute(&self, input: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
    if input.is_empty() {
        return Err(HandlerError::ExecutionFailed("agent skill input must not be empty".to_string()));
    }
    Ok(format!("{}:{}", self.skill, input))  // format string, not real skill dispatch
}
```

Real skill dispatch happens in `DefaultAgent::execute_skill()`, which is a separate code path that bypasses the handler dispatch chain entirely. It constructs its own `HandlerContext` with `SecurityContext::unauthenticated()`, losing the caller's security context:

```rust
// This context creation is a construction site that must be updated
let ctx = HandlerContext::new(&SecurityContext::unauthenticated(), &StdCommandBusFactory::direct());
skill.execute(input, ctx).await
```

### ProviderFactory::provider() returns a concrete type

```rust
// Current signature
fn provider(config: ProviderConfig, model: ModelInfo, completer: Arc<dyn Completer>) -> ProviderCore;
```

The return type is `ProviderCore` (concrete, pub(crate)), not `Arc<dyn Provider>`. This must change to `Arc<dyn Provider>` before the observer seam can be added — a provider stored as a concrete type cannot be shared across request lifetimes without wrapping.

### No LLM crate depends on edge-domain-observer

Confirmed `Cargo.toml` for all five LLM crates: none lists `edge-domain-observer` as a dependency. The `ObserveContext` trait named in this ADR does not yet exist anywhere in the codebase.

---

## Decision: two injection seams

Rather than stitching observability into each LLM crate independently (which would scatter coupling), this ADR establishes exactly two seams — one in the handler dispatch graph, one in the provider construction path. Every LLM primitive flows through one or both.

---

### Seam 1 — `HandlerContext` carries `ObserveContext`

**New trait in `edge-domain-observer`:**

```rust
pub trait ObserveContext: Send + Sync {
    fn tracer(&self) -> &dyn HandlerTracer;
    fn drain(&self) -> &dyn LogDrain;
    fn metrics(&self) -> &dyn MetricRegistry;
}
```

`ObserveContext` is a composition handle — a single object the runtime injects once that gives handler logic access to all three observability ports. It does not own the ports; it borrows them. Implementers hold `Arc<dyn HandlerTracer>`, `Arc<dyn LogDrain>`, `Arc<dyn MetricRegistry>` and return references to their inner data.

**Change to `HandlerContext` in `edge-domain-handler`:**

```rust
// HandlerContext remains an enum — observer is added to the Standard variant
#[derive(Copy, Clone)]
pub enum HandlerContext<'a> {
    Standard {
        security: &'a SecurityContext,
        commands: &'a dyn CommandBus,
        observer: &'a dyn ObserveContext,   // NEW
    },
}

impl<'a> HandlerContext<'a> {
    pub fn new(
        security: &'a SecurityContext,
        commands: &'a dyn CommandBus,
        observer: &'a dyn ObserveContext,   // NEW
    ) -> Self {
        Self::Standard { security, commands, observer }
    }

    pub fn observer(&self) -> &dyn ObserveContext {
        match self { Self::Standard { observer, .. } => *observer }
    }
}
```

`observer` uses `&'a dyn ObserveContext` — same lifetime discipline as `security` and `commands`. The `Copy + Clone` derive is preserved: the type holds only references.

**Coverage:** every `Skill`, `DefaultAgentHandler`, `DefaultPromptHandler`, `DefaultReasoningHandler`, `DefaultProviderHandler`, and any future `Handler` impl automatically receives the observer. Handlers call `ctx.observer().tracer()` to open a span, `ctx.observer().drain()` to emit a log record, or `ctx.observer().metrics()` to record a counter.

**Noop path:** `NoopObserveContext` is unconditionally available from `edge-domain-observer`'s SAF. Tests construct `HandlerContext::new(security, commands, &NoopObserveContext)`.

---

### Seam 2 — `ProviderFactory` returns `Arc<dyn Provider>` and accepts `Arc<dyn ObserveContext>`

Two changes are required together — the return type must be sealed behind the trait before the observer parameter can be stored inside the `Provider` impl:

```rust
pub trait ProviderFactory {
    fn provider(
        config: ProviderConfig,
        model: ModelInfo,
        completer: Arc<dyn Completer>,
        observer: Arc<dyn ObserveContext>,   // NEW
    ) -> Arc<dyn Provider>;                  // was: -> ProviderCore
}
```

`ProviderCore` is replaced by `Arc<dyn Provider>` as the return type. `ProviderCore` becomes the internal implementation detail that `StdProviderFactory` wraps in `Arc::new(...)`. Callers never name `ProviderCore`.

The `Provider` impl stores `Arc<dyn ObserveContext>` and uses it inside `execute_step()` to:

- Increment a `Counter` for total completion calls.
- Record a `Histogram` observation for round-trip latency (duration of the `Completer::complete()` call).
- Emit a `LogRecord` with `model_name`, `prompt_tokens`, `completion_tokens` on success.
- Emit a `LogRecord` with `error_kind`, `model_name` on failure.

**Why `Arc` here vs `&'a dyn` in Seam 1:** `Provider` impls are long-lived (stored in `HandlerRegistry`, shared across requests). `Arc` is the only safe handle. `HandlerContext` is ephemeral (constructed per dispatch call), so borrowed references suffice and avoid a heap allocation on the hot path.

---

## What this is NOT

- **Not an OTel SDK integration.** `ObserveContext` is a pure domain trait. The OTel subscriber, Prometheus exporter, and OTLP exporter remain in `edge-observe` (future assembler-layer crate). This ADR only establishes the domain-level injection points.

- **Not a replacement for `edge-dispatch`'s pipeline span.** The `"pipeline.stage"` span that `edge-dispatch` creates covers the router-to-handler boundary. `HandlerTracer` spans created via `ctx.observer().tracer()` are children of that span — they cover handler-internal operations.

- **Not a tracing framework.** No propagation, no context extraction, no W3C traceparent. The domain layer creates and records child spans; the infrastructure layer (OTel subscriber) emits them.

- **Not a fix for `DefaultAgentHandler`'s stub implementation.** This ADR threads observability through the existing dispatch paths. Replacing `DefaultAgentHandler`'s format-string stub with real skill dispatch is a separate concern tracked as L4 below.

---

## Dependency changes

| Crate | Change | Dep added |
|---|---|---|
| `edge-domain-observer` | New `ObserveContext` trait + `NoopObserveContext` struct | None |
| `edge-domain-handler` | `HandlerContext::Standard` variant gains `observer`; `new()` updated | `edge-domain-observer` |
| `edge-llm-provider` | `ProviderFactory::provider` return type → `Arc<dyn Provider>`; gains `observer` param | `edge-domain-observer` |
| `edge-llm-agent` | `DefaultAgentHandler::execute` removes `_ctx` prefix; uses `ctx.observer()` | Transitive via handler |
| `edge-llm-agent` | `DefaultAgent::execute_skill` propagates caller's `HandlerContext` to skills | None new |
| `edge-llm-prompt` | `DefaultPromptHandler::execute` removes `_ctx` prefix; uses `ctx.observer()` | Transitive via handler |
| `edge-llm-reasoning` | `DefaultReasoningHandler::execute` removes `_ctx` prefix; uses `ctx.observer()` | Transitive via handler |
| `edge-llm-provider` | `DefaultProviderHandler::execute` removes `_ctx` prefix; uses `ctx.observer()` | Transitive via handler |

`edge-llm-complete` is unchanged — it defines the `Completer` port but does not dispatch and does not need observability context at the domain level.

---

## Boundary rules

**B1 — `ObserveContext` is defined in `edge-domain-observer` only.** No LLM crate defines a parallel observe-context type. All LLM crates depend on `edge-domain-observer` for this trait.

**B2 — `HandlerContext` is the sole dispatch carrier.** Observability context must not be threaded through method parameters separately. Handlers receive one context object; they access the observer through it.

**B3 — Provider observability is emission-only.** The `Provider` impl records metrics and log records. It does not create top-level spans — it emits child spans via `tracer().start_span(...)`. Span propagation is an infrastructure concern.

**B4 — Noop impls are unconditionally available.** `NoopObserveContext` must compile without feature flags. Any handler test that constructs a `HandlerContext` uses `NoopObserveContext` by default.

**B5 — No SDK deps in domain crates.** `edge-domain-observer`, `edge-domain-handler`, and all `edge-llm-*` crates must not import OTel, Prometheus, or any metrics SDK. Port contracts only.

**B6 — `ProviderCore` stays `pub(crate)`.** The return type change to `Arc<dyn Provider>` makes `ProviderCore` fully internal. No external code must name `ProviderCore` after this change.

---

## Construction site changes

**`HandlerContext::new` call sites** — every location that constructs a `HandlerContext` must add an observer argument. Primary sites:

| Site | File | Observer to pass |
|---|---|---|
| `edge-dispatch` | Router dispatch loop | Runtime-injected `Arc<dyn ObserveContext>` |
| `DefaultAgent::execute_skill` | `core/types/default_agent.rs` | Forward the caller's `ctx` instead of constructing a new one |
| All LLM integration tests | `tests/*.rs` in each LLM crate | `&NoopObserveContext` |
| `edge-domain-handler` tests | `tests/*.rs` | `&NoopObserveContext` |

`DefaultAgent::execute_skill` is the most important site — it currently constructs its own `HandlerContext` with `SecurityContext::unauthenticated()`, silently dropping the caller's security principal. The fix is to accept `HandlerContext<'_>` as a parameter and forward it to `skill.execute()`.

**`ProviderFactory::provider()` call sites** — every location that calls `StdProviderFactory::provider(config, model, completer)` must add a `NoopObserveContext` (tests) or a runtime-injected observer (assembler).

---

## Known limitations (deferred)

**L1 — `ObserveContext` has no context propagation.** W3C traceparent and baggage threading are assembler concerns, not domain concerns. The domain creates spans; the OTel subscriber propagates them. If distributed trace correlation across service boundaries is required, it must be injected via a separate `PropagationContext` — not layered onto `ObserveContext`. Tracked: TBD.

**L2 — `Completer::complete()` latency is measured at the `Provider` level, not the `Completer` level.** A slow completer that is called from multiple providers will be attributed to the provider, not to the underlying HTTP transport. SDK-backed `Completer` impls that instrument themselves individually are a future concern for `edge-observe`. Tracked: TBD.

**L3 — No per-skill latency histogram.** Seam 1 gives each handler access to `metrics()`, but `Skill` dispatch inside `DefaultAgent::execute_skill` is not separately instrumented at the skill boundary — only the outer `agent_handler` call is. Per-skill histograms require each `Skill::execute` to receive a child span opened from the parent context. This is the correct long-term design but is deferred. Tracked: TBD.

**L4 — `DefaultAgentHandler::execute` is a stub.** The current implementation returns `format!("{}:{}", self.skill, input)` — it does not resolve or execute a real `Skill`. Real agent dispatch flows through `DefaultAgent::execute_skill()`, which is a separate path. Replacing the stub with real skill resolution via an `Arc<dyn Agent>` reference is a separate work item. This ADR instruments the stub path so that when the real implementation lands, observability is already wired. Tracked: TBD.

**L5 — `DefaultAgent::execute_skill` drops the caller's security context.** This is fixed by this ADR (see construction site changes), but the fix requires `execute_skill` to accept `HandlerContext<'_>` as a parameter — a signature change that affects the `Agent` trait itself and all `Agent` implementors. This is a larger ripple than the seam changes and may be sequenced separately. Tracked: TBD.

---

## Implementation order (layer-gated TDD)

1. **`ObserveContext` + `NoopObserveContext` in `edge-domain-observer`**  
   Add `api/observe/traits/observe_context.rs`; add `core/observe/noop/noop_observe_context.rs`; factory fn `noop_observe_context()` in `saf/observe/observe_context_svc.rs`; export from `lib.rs` and `saf/mod.rs`.  
   Gate: `cargo test`, `arch audit --rs` 183/183, `cargo clippy -D warnings`.

2. **`HandlerContext` seam in `edge-domain-handler`**  
   Add `edge-domain-observer` to `Cargo.toml`; add `observer: &'a dyn ObserveContext` to `HandlerContext::Standard`; update `new()` and `observer()` accessor; update all construction sites in `edge-domain-handler`'s own tests to pass `&NoopObserveContext`.  
   Gate: `cargo test`, `arch audit --rs`, `cargo clippy -D warnings`.

3. **`ProviderFactory` seam in `edge-llm-provider`**  
   Add `edge-domain-observer` to `Cargo.toml`; change `provider()` return type from `ProviderCore` to `Arc<dyn Provider>`; add `observer: Arc<dyn ObserveContext>` param; store in `ProviderCore`; emit counter + histogram + log records in `execute_step()`; update `StdProviderFactory`; update all integration test call sites.  
   Gate: `cargo test`, `arch audit --rs`, `cargo clippy -D warnings`.

4. **Handler instrumentation across LLM crates**  
   Remove `_ctx` prefix in `DefaultProviderHandler`, `DefaultReasoningHandler`, `DefaultPromptHandler`, `DefaultAgentHandler`; open a child span and emit a completion log record via `ctx.observer()` in each; update construction sites in integration tests to pass `&NoopObserveContext` via `HandlerContext::new`.  
   Gate: `cargo test`, `arch audit --rs`, `cargo clippy -D warnings`, `cargo fmt --check`.

5. **`DefaultAgent::execute_skill` context propagation** _(may be sequenced separately — see L5)_  
   Change `Agent::execute_skill(&self, skill_name: &str, input: String)` to `execute_skill(&self, skill_name: &str, input: String, ctx: HandlerContext<'_>)`; remove internal `HandlerContext` construction from `DefaultAgent::execute_skill`; forward caller's `ctx` to `skill.execute()`; update all `Agent` implementors and call sites.  
   Gate: same as step 4.

Each step is blocked on the previous — `HandlerContext` cannot be updated before `ObserveContext` is defined; `ProviderFactory` and handler instrumentation cannot be updated before `HandlerContext` carries the observer.
