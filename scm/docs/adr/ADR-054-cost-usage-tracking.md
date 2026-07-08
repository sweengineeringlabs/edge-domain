# ADR-054: Cost / Usage Tracking for LLM Completions

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-044 (Observability↔LLM Integration), ADR-045 (`edge-llm-runtime` Composition Root — the 2026-07-08 audit that first named this gap), ADR-046 (`edge-llm-tools` Governance — same reuse pattern)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit (the same pass that produced ADR-045/046) lists, verbatim, among the things a composition root does not fix: *"…cost/usage tracking, eval harness…"*. Confirmed independently here: zero cost/usage/billing aggregation code exists anywhere in `edge/` — no counter, no ledger, no per-tenant rollup, nothing that survives past a single call.

That is not for lack of raw material. `edge-llm-provider`'s `TokenUsage` (`domain/scm/domain/llm/provider/main/src/api/provider/types/token_usage.rs`) already carries `prompt_tokens`, `completion_tokens`, `total_tokens`, `cache_read_input_tokens`, `cache_creation_input_tokens`, plus `total_with_cache()` and `cache_hit()` helpers (`core/provider/token_usage.rs`). `edge_llm_complete::CompletionResponse.usage: Box<TokenUsage>` (`domain/scm/domain/llm/complete/main/src/api/complete/types/completion_response.rs`) is a **structurally identical but separately-defined** `TokenUsage` in a different crate — `EchoProviderCompleter::complete` (`provider/main/src/core/provider/provider_completer.rs:136-139`) already builds a real, non-zero one per call from `result.tokens_used`. So a genuine per-call usage value exists at runtime today; it simply evaporates the moment the call returns.

Two related, pre-existing gaps surfaced during this research, worth naming even though this ADR doesn't fix them:
- `Provider::last_token_usage()` (`std_provider.rs:101-108`) is a stub — it unconditionally returns `TokenUsage::new(0, 0, 0, 0)`, never the value actually computed by the completer. "Per-call usage" isn't even wired to its own single-slot accessor yet, let alone aggregated.
- `edge_llm_provider::TokenUsage` and `edge_llm_complete::TokenUsage` are two distinct types with the same five fields and no `From`/`Into` between them; `provider_completer.rs` re-imports the `complete`-crate one and constructs it directly rather than converting from the `provider`-crate one.

The natural aggregation dimension is tenant, and it is already available with **zero new plumbing**: `SecurityContext` (`edge-security-runtime`) carries `tenant_id: Option<String>` and is a required field of `HandlerContext` (`domain/scm/domain-handler/main/src/api/handler/types/handler_context.rs:14`), threaded into every `Handler::execute` call today. `ObserverContext` (`edge-domain-observer`, proven wired per ADR-044) is likewise already in `HandlerContext.observer`, and its `MetricRegistry::counter(...).increment(IncrementRequest{ delta })` is exactly the emission mechanism ADR-046 already uses for governance decisions. Per the same reuse discipline ADR-046 applied to `Policy`/`SecurityContext`/`ObserverContext`, this ADR does not invent a new context or audit type — it reuses these three.

## Decision

Build a small new crate, **`edge-llm-usage`**, with two ports of deliberately different maturity:

1. **`UsageRecorder`** (built now, v1) — records one completion's `TokenUsage` as an `ObserverContext` metric. No persistence, no query surface — the same "emit through the observability plane, don't add a bespoke store" move ADR-046 made for governance decisions.
2. **`UsageLedger`** (contract only, not implemented here) — the port a real aggregation backend would implement. Only a `NoopUsageLedger` ships in this crate. A persistent backend is a plugin/spi concern per ADR-042's boundary and is explicitly future work, not this ADR's scope.

### Shape / workspace layout

```
domain/scm/domain/llm/usage/                    (edge-llm-usage)
├── api/
│   ├── traits/
│   │   ├── usage_recorder.rs      pub trait UsageRecorder
│   │   └── usage_ledger.rs        pub trait UsageLedger   (contract only — see below)
│   ├── types/
│   │   ├── usage_record_request.rs    { usage: TokenUsage, model: String, tenant_id: Option<String> }
│   │   ├── usage_record_response.rs   (ack marker)
│   │   ├── usage_append_request.rs    { usage: TokenUsage, model: String, tenant_id: Option<String>, recorded_at_ms: u64 }
│   │   ├── usage_append_response.rs   (ack marker)
│   │   ├── usage_totals_request.rs    { tenant_id: Option<String>, model: Option<String> }
│   │   └── usage_totals_response.rs   { total_tokens: u64, call_count: u64 }
│   └── errors/usage_error.rs      pub enum UsageError { RecorderUnavailable(String), LedgerUnavailable(String) }
├── core/
│   └── default_usage_recorder.rs  DefaultUsageRecorder — ObserverContext-backed UsageRecorder impl
└── spi/
    └── noop_usage_ledger.rs       NoopUsageLedger — accepts writes, answers zeroed totals; no storage
```

```rust
pub trait UsageRecorder: Send + Sync {
    /// Emit one completion's token usage against a model/tenant dimension.
    fn record(&self, req: UsageRecordRequest) -> Result<UsageRecordResponse, UsageError>;
}

pub trait UsageLedger: Send + Sync {
    /// Persist one usage observation. Real backends aggregate; `NoopUsageLedger` discards.
    fn append(&self, req: UsageAppendRequest) -> Result<UsageAppendResponse, UsageError>;
    /// Read back an aggregate — `NoopUsageLedger` always answers zero.
    fn totals(&self, req: UsageTotalsRequest) -> Result<UsageTotalsResponse, UsageError>;
}
```

`DefaultUsageRecorder::record` dimensions the metric by encoding `model`/`tenant_id` into the counter's name string (the same convention ADR-046 uses for `tool_governance_checked_total{allowed=...}` — `MetricRegistry::counter` takes a flat `name: String`, not structured labels, so there is no richer mechanism available today):

```rust
fn record(&self, req: UsageRecordRequest) -> Result<UsageRecordResponse, UsageError> {
    let tenant = req.tenant_id.as_deref().unwrap_or("unknown");
    let name = format!("llm.tokens.total.model.{}.tenant.{}", req.model, tenant);
    let counter = self.observer.metrics(MetricsRequest)?.registry
        .counter(CounterLookupRequest { name })?.counter;
    counter.increment(IncrementRequest { delta: req.usage.total_tokens as u64 })?;
    Ok(UsageRecordResponse)
}
```
(Known limitation, accepted for v1 the same way ADR-046 accepted it: per-tenant counter names have unbounded cardinality against a real metrics backend. Not a blocker — no such backend is wired anywhere yet either.)

### Recording seam

`StdProvider::complete()` (`provider/main/src/core/provider/std/std_provider.rs:134-150`) is the exact point a real `TokenUsage` first exists: immediately after `self.completer.complete(...)` returns `Ok(response)`, `response.usage` is populated (as `EchoProviderCompleter` already demonstrates). This ADR adds one field to `StdProvider` — `usage_recorder: Arc<dyn UsageRecorder>` — alongside its existing `observer` field, and one call after the completer succeeds:

```rust
async fn complete(&self, req: ProviderCompleteRequest) -> Result<ProviderCompletionResponse, ExecutionError> {
    let model = self.model_info(ModelInfoLookupRequest)?.info.id.clone();
    // ... existing temperature/request-building unchanged ...
    let response = self.completer.complete(CompleteRequest { request: &request }).await?;
    let _ = self.usage_recorder.record(UsageRecordRequest {
        usage: (*response.usage).clone().into(), // provider::TokenUsage ⇄ complete::TokenUsage — see below
        model,
        tenant_id: req.tenant_id.clone(),
    });
    Ok(ProviderCompletionResponse { response })
}
```

`Provider::complete`'s signature (`ProviderCompleteRequest`) carries no `SecurityContext` today — `Provider` is deliberately a lower, ctx-free trait, unlike `Handler`. Threading a full `SecurityContext` through it would ripple into every `Provider` impl for a concern `Handler` already carries — the opposite of this ADR's reuse discipline. Instead, this is the **one small, additive plumbing change** this ADR makes: add `tenant_id: Option<String>` to `ProviderCompleteRequest` itself (a request-field addition, not a trait-shape change), populated by whichever `Handler` calls `Provider::complete()` from `req.ctx.security.tenant_id` — reusing the existing field, not a new context type. No `Provider` implementation other than `StdProvider` needs to change beyond accepting the new field (it defaults via `..Default::default()`-style construction where applicable).

A recording failure never fails the completion (`let _ =` above, mirroring how `HandlerContext.observer` calls elsewhere already treat metrics/tracing as best-effort) — usage tracking must not become a new way for `Provider::complete` to fail.

## What this ADR explicitly does NOT solve

- **No real persistent `UsageLedger` backend.** `NoopUsageLedger` is the only implementation shipped. A Postgres/Redis/etc.-backed ledger is a plugin/spi concern per ADR-042's boundary and is separate follow-on work, not silently assumed by this ADR — exactly how ADR-045 kept "real vendor `Completer`" out of its own scope.
- **No `SpendLimitPolicy` / budget enforcement.** Once a real `UsageLedger` exists, a policy reusing `edge_domain_policy::Policy<Input = SpendCheckRequest>` (consulting `UsageLedger::totals` against a per-tenant ceiling) is the natural next step, composed exactly like ADR-046's `CapabilityGatePolicy`/`RiskCeilingPolicy`. Not designed here — needs a real ledger to consult first.
- **No token→currency conversion.** `UsageRecorder`/`UsageLedger` deal only in raw token counts; a pricing catalog (cost per token per model) does not exist anywhere in `edge/` and is out of scope.
- **No end-to-end HTTP-observable usage.** `edge-llm-runtime` (ADR-045) is not built yet; this ADR wires the recording call inside `StdProvider::complete()`, not through a live ingress path. Usage becomes observable over real traffic only once ADR-045 exists and a `Handler` actually calls `Provider::complete()` (today, no `Handler` in the repo does — `DefaultProviderHandler` calls `ExecutionModel::execute_step` instead; see Alternatives Considered).
- **Does not fix `last_token_usage()`'s pre-existing stub bug** (always returns zeroed usage). Related, but a separate, narrower defect — tracked, not fixed here.
- **Does not unify `edge_llm_provider::TokenUsage` and `edge_llm_complete::TokenUsage`.** Two structurally identical, separately-defined types are a pre-existing duplication this ADR works around (via `.into()` at the recording call site) rather than resolves.
- **No streaming usage.** `Provider::completer()`/`Completer::complete_stream` don't carry a final `TokenUsage` the way `complete()` does; recording streamed completions is deferred.

## Consequences

**What this enables**
- A real, swappable emission port for LLM usage (`UsageRecorder`), populated from data that already exists per-call, visible in the same observability plane as everything else (ADR-044) — no bespoke audit log, no dead struct.
- A stable, small `UsageLedger` contract that a future persistent backend can implement without any call-site changes — `StdProvider` and any `Handler` depend only on the trait.
- The dimension needed for per-tenant attribution (`tenant_id`) is threaded with a single additive field on an existing request type, not a new context concept.
- A concrete, named next step (`SpendLimitPolicy`) that slots into the exact `Policy`/`CompositePolicy` machinery ADR-046 already proved out, once a real ledger exists to back it.

**What this requires**
- New crate `edge-llm-usage` under `domain/scm/domain/llm/usage/`.
- One new field, `tenant_id: Option<String>`, on `edge_llm_provider::ProviderCompleteRequest`.
- One new field, `usage_recorder: Arc<dyn UsageRecorder>`, on `StdProvider`, and one call site inside `StdProvider::complete()`.
- A `TokenUsage` conversion (`edge_llm_complete::TokenUsage` → `edge_llm_provider::TokenUsage`, or a shared `From` impl) at that call site — the pre-existing duplication becomes visible the moment usage is actually consumed instead of merely passed through.
- No changes to `edge-domain-handler`, `edge-domain-observer`, `edge-security-runtime`, or `Provider`'s trait method signatures (only the `ProviderCompleteRequest` data type gains a field).

## Alternatives Considered

**Track usage as an in-memory field on `StdProvider` itself (e.g. a `Mutex<HashMap<String, u64>>` counter)**
Rejected. Not observable or exportable through any existing channel, has no reset/window semantics, and duplicates what `ObserverContext`'s `MetricRegistry` already gives for free. Also ties usage data to a single `StdProvider` instance's lifetime rather than a swappable port — the opposite of treating usage tracking as a pluggable concern.

**Thread the full `SecurityContext` into `Provider::complete()`**
Rejected. `Provider` is deliberately a lower, ctx-free trait — none of its other methods take a `HandlerContext`. Adding one parameter for this alone would ripple through every `Provider` impl and call site for a concern `Handler` already carries via `req.ctx.security`. A single `tenant_id: Option<String>` field on `ProviderCompleteRequest` gets the same dimension in with far smaller blast radius.

**Build the persistent `UsageLedger` backend now (e.g., an in-memory `HashMap`-backed `core::InMemoryUsageLedger`)**
Rejected as premature, not merely deferred. A real store is explicitly a plugin/spi concern per ADR-042, and shipping even an in-memory version risks it being mistaken for "the" backend and never replaced, exactly the trap ADR-045 named for "echo `Completer`s" being mistaken for a real vendor integration. Ship only the contract plus `NoopUsageLedger`.

**Fix and extend `last_token_usage()` into an aggregator instead of adding a new port**
Rejected. It is already a stub returning hardcoded zero — a separate, narrower defect. Even fixed, "last" is single-slot state by construction, not cross-call aggregation; making it aggregate would require it to grow into an internal `Vec`/store — which is exactly what `UsageLedger` already is meant to be, better done as a first-class, testable port than a hidden field bolted onto `Provider`.

## Tracking

- New crate: `edge-llm-usage` (`domain/scm/domain/llm/usage/`)
- Add `tenant_id: Option<String>` to `edge_llm_provider::ProviderCompleteRequest`; wire `StdProvider::complete()` to call `UsageRecorder::record(...)` after the completer succeeds
- Fix pre-existing defect (separate, smaller, unrelated to this ADR's build): `Provider::last_token_usage()` always returns zeroed `TokenUsage` regardless of the real value computed per call
- Cleanup (pre-existing duplication, separate issue): unify or bridge `edge_llm_provider::TokenUsage` and `edge_llm_complete::TokenUsage`
- Follow-up (separate ADR/issue, explicitly out of scope here): a real persistent `UsageLedger` backend, per ADR-042's plugin boundary
- Follow-up (not designed here — needs a real ledger first): `SpendLimitPolicy` via `edge_domain_policy::Policy`, composed the same way ADR-046 composed `CapabilityGatePolicy`/`RiskCeilingPolicy`
- Depends on `edge-llm-runtime` (ADR-045) existing, and some `Handler` actually calling `Provider::complete()`, before usage is observable over real traffic
