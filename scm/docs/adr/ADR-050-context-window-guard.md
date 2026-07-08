# ADR-050: Proactive Context-Window Guard for `edge-llm-provider`

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-034 (LLM Prompt), ADR-043 (LLM Complete), ADR-046 (edge-llm-tools Governance — same Policy-reuse pattern)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit (carried in ADR-045's "What this ADR explicitly does NOT solve") confirmed: **no context-window overflow prevention exists anywhere in `edge/`.** Two pieces exist in isolation and have never been joined:

- `edge_llm_prompt::TokenCounter` (`domain/scm/domain/llm/prompt/main/src/api/prompt/traits/token_counter.rs`) — `count_tokens`, `estimate_tokens`, `tokenizer_name`, `is_exact`, all in the mandatory Request/Response shape. Nothing in `edge-llm-provider` or `edge-llm-complete` calls it.
- `ModelInfo.context_window` — and there are **two distinct `ModelInfo` types**, not one:
  - `edge_llm_provider::api::provider::types::ModelInfo` (`provider/main/src/api/provider/types/model_info.rs`) — fields `id, name, family, context_window, supports_vision, supports_functions, supports_streaming, training_cutoff`. This is the one `StdProvider` actually holds (`pub(crate) model: Option<ModelInfo>`, `provider/main/src/api/provider/types/std_provider.rs:17`).
  - `edge_llm_complete::api::complete::types::ModelInfo` (`complete/main/src/api/complete/types/model_info.rs`) — a *different*, structurally-similar type (`id, name, provider, context_window, supports_vision, supports_function_calling, supports_streaming`), unrelated by any `impl From`/`impl Into`, never read by `StdProvider`. This duplication is a pre-existing smell this ADR does not fix (see "What this ADR explicitly does NOT solve") — the guard below is wired against the **provider crate's own `ModelInfo`**, since that is the instance in scope at the only call site that matters.

`StdProvider::complete()` (`provider/main/src/core/provider/std/std_provider.rs:134-150`) today:
```rust
async fn complete(&self, req: ProviderCompleteRequest) -> Result<ProviderCompletionResponse, ExecutionError> {
    let model = self.model_info(ModelInfoLookupRequest)?.info.id.clone();
    let temperature = self.provider_config(ProviderConfigLookupRequest)?.config.temperature;
    let request = req.input.into_completion_request(model, temperature);

    self.completer
        .complete(CompleteRequest { request: &request })
        .await
        .map(|response| ProviderCompletionResponse { response })
        .map_err(ExecutionError::from)
}
```
It calls `model_info()` and `provider_config()` — both already read `self.model`/`self.config` — but the resulting `context_window` value is discarded; nothing compares it to the assembled request's token count before handing off to `self.completer.complete(...)` (a network call for any real vendor `Completer`). `ExecutionError::ContextWindowExceeded { max_tokens, requested }` (`provider/main/src/api/provider/errors/execution_error.rs:19-24`) already exists as a variant, but grepping the crate shows it is only ever produced **reactively**, mapped from a vendor's own `CompleteError::ContextLengthExceeded` after the round-trip already happened. Every context-window overflow today costs a wasted network call (and, for paid vendor APIs, a wasted charge) before the same information could have been known locally.

`StdProvider` has **no existing seam** for a check of this kind: its fields are exactly `config, model, completer, observer` (`provider/main/src/api/provider/types/std_provider.rs:15-20`), and its only constructor is `StdProvider::new(config, model, completer, observer)` (`provider/main/src/core/provider/std/std_provider.rs:21-33`) — no `CompositePolicy` field, no `TokenCounter` field. Any fix requires a constructor change no matter how the check is implemented.

## Decision

Add a proactive `ContextWindowPolicy` — an `edge_domain_policy::Policy<Input = ContextWindowCheckInput>` — evaluated inside `StdProvider::complete()` immediately after the request is assembled and before `self.completer.complete(...)` is called. On violation, map the resulting `PolicyError` to the existing `ExecutionError::ContextWindowExceeded { max_tokens, requested }` and return it — no network call is made.

This follows ADR-046's discipline exactly: reuse `edge_domain_policy::Policy`/`CompositePolicy`/`PolicyError` as the enforcement mechanism, don't invent a new gate type. A naive `ContextWindowGuard` struct with its own bespoke error and its own ad-hoc invocation point would duplicate a port that already exists, satisfies `api_error_type_named`, and is already composable.

### Why `Policy`-based, not inlined

`StdProvider::complete()` has no existing seam to run any check — `Policy`-based or inline both require a constructor signature change, so "avoid touching the constructor" is not an argument for inlining. Given that a breaking change to `StdProvider::new` is unavoidable either way, the marginal cost of routing through `Policy` is close to zero, and it buys real things an inline `if` would not:

- **Uniform violation type.** `PolicyError` is already the shape every other cross-cutting business rule in this codebase surfaces through (ADR-046's `CapabilityGatePolicy`/`RiskCeilingPolicy`). An inline check would invent its own ad-hoc `if estimated + max_tokens_per_call > context_window { return Err(...) }` with no independent name, no independent test seam, and no way to compose with a future second numeric gate (e.g. a cost-ceiling check) without further `complete()` edits.
- **Independent testability.** `ContextWindowPolicy::evaluate` can be unit-tested against `ContextWindowCheckInput` values directly, with no `StdProvider`, `Completer`, or `Arc<dyn TokenCounter>` construction required — exactly the same benefit ADR-046 cites for `CapabilityGatePolicy`.
- **Consistency, not novelty.** This crate family has now standardized on "cross-cutting rule = `Policy` impl" twice (tool governance, this ADR). A third numeric gate (e.g. output-length capping) has an obvious home instead of a bespoke third pattern.

No new crate is warranted (unlike ADR-046's `edge-llm-tools`): this is a single, self-contained rule over types (`ModelInfo`, `ProviderCompleteRequest`, `ExecutionError`) that already belong to `edge-llm-provider`. It lives inside that crate, not in a new one.

### Dependency check: does `edge-llm-provider` need `edge-llm-prompt`?

**Yes — and it does not create a cycle.** Verified by reading both `Cargo.toml`s:

- `provider/Cargo.toml` depends on `edge-domain-handler`, `edge-domain-observer`, `swe-edge-configbuilder`, `edge-llm-complete`. It does **not** depend on `edge-llm-prompt` today.
- `prompt/Cargo.toml` depends on `edge-domain-handler`, `edge-domain-observer` only. It does **not** depend on `edge-llm-provider`, and does not depend on `edge-llm-complete` either.
- `complete/Cargo.toml` depends on `edge-pipeline` only — no dependency on `prompt` or `provider`.

So the existing graph is `provider → complete`, with `prompt` and `complete` both leaves relative to each other. Adding `provider → prompt` produces `provider → {prompt, complete}`, still a DAG — no cycle. This must be added to `provider/Cargo.toml`'s `[dependencies]` as `edge-llm-prompt.workspace = true`.

### Shape

No new crate. New files inside `edge-llm-provider`:

```
provider/main/src/
├── api/provider/types/
│   └── context_window_check_input.rs   # ContextWindowCheckInput (Copy, no lifetimes)
├── core/provider/
│   └── policies/
│       └── context_window_policy.rs    # ContextWindowPolicy: Policy<Input = ContextWindowCheckInput>
```

`ContextWindowCheckInput` — a small, `Copy`, request-shaped bundle (kept out of `Policy::evaluate`'s generic `PolicyEvaluateRequest<'_, I>` itself, per the port's own shape):
```rust
/// Bundled inputs for ContextWindowPolicy::evaluate — deliberately numeric-only
/// (no CompletionInput/ModelInfo borrow) so the policy has no lifetime coupling
/// to the request being assembled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextWindowCheckInput {
    pub estimated_prompt_tokens: u32,
    pub max_tokens_per_call: u32,
    pub context_window: u32,
}
```

`ContextWindowPolicy` (core/, mirrors `CapabilityGatePolicy`/`RiskCeilingPolicy` from ADR-046):
```rust
impl Policy for ContextWindowPolicy {
    type Input = ContextWindowCheckInput;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "context-window" })
    }

    fn evaluate(&self, req: PolicyEvaluateRequest<'_, Self::Input>) -> Result<(), PolicyError> {
        let input = req.input;
        let total = input.estimated_prompt_tokens.saturating_add(input.max_tokens_per_call);
        if total > input.context_window {
            return Err(PolicyError::new(
                "context-window",
                format!(
                    "estimated {total} tokens (prompt {} + max_tokens_per_call {}) exceeds context window {}",
                    input.estimated_prompt_tokens, input.max_tokens_per_call, input.context_window
                ),
            ));
        }
        Ok(())
    }
}
```

`StdProvider` gains one new required field/constructor param — `token_counter: Arc<dyn edge_llm_prompt::TokenCounter>` — and builds/holds a `context_policy: CompositePolicy<ContextWindowCheckInput>` (a single-policy composite today; the extension point ADR-046 already relies on, so a future cost-ceiling or output-length policy composes in with zero further `StdProvider` changes):
```rust
pub struct StdProvider {
    pub(crate) config: ProviderConfig,
    pub(crate) model: Option<ModelInfo>,
    pub(crate) completer: Arc<dyn Completer>,
    pub(crate) observer: Arc<dyn ObserverContext>,
    pub(crate) token_counter: Arc<dyn TokenCounter>,               // new
    pub(crate) context_policy: CompositePolicy<ContextWindowCheckInput>, // new
}
```

`complete()` becomes:
```rust
async fn complete(&self, req: ProviderCompleteRequest) -> Result<ProviderCompletionResponse, ExecutionError> {
    let info = self.model_info(ModelInfoLookupRequest)?.info;
    let model = info.id.clone();
    let temperature = self.provider_config(ProviderConfigLookupRequest)?.config.temperature;
    let request = req.input.into_completion_request(model, temperature);

    // Proactive guard — before any network call.
    let estimated = self.token_counter.estimate_tokens(EstimateTokensRequest {
        text: &request.prompt_text(), // system + messages joined; see note below
    })?.count as u32;
    let check = ContextWindowCheckInput {
        estimated_prompt_tokens: estimated,
        max_tokens_per_call: request.max_tokens,
        context_window: info.context_window,
    };
    if let Err(policy_err) = self.context_policy.evaluate(PolicyEvaluateRequest { input: &check }) {
        let _ = policy_err; // reason already carries the human-readable detail; log via self.observer
        return Err(ExecutionError::ContextWindowExceeded {
            max_tokens: info.context_window,
            requested: estimated.saturating_add(check.max_tokens_per_call),
        });
    }

    self.completer
        .complete(CompleteRequest { request: &request })
        .await
        .map(|response| ProviderCompletionResponse { response })
        .map_err(ExecutionError::from)
}
```
Note: `PolicyError`'s fields (`policy: &'static str`, `reason: String`) are not parsed back out — `complete()` already has `info.context_window` and `estimated`/`max_tokens_per_call` in scope from building `check`, so the existing, structured `ExecutionError::ContextWindowExceeded { max_tokens, requested }` is constructed directly from those values, not by parsing the policy's string reason. `Policy::evaluate` is purely the enforcement decision (pass/fail + a human-readable `reason` for logs/observability via `self.observer`); reconstructing the domain-specific error stays the caller's job, exactly as `GovernedHandler` does in ADR-046 (`HandlerError` wraps `PolicyError` directly there only because no richer domain error existed yet — here one already does, so it is preferred over a generic wrap).

`request.prompt_text()` is a small new helper (or a free function) that joins `CompletionInput.system` and each `CompletionMessage`'s content — estimation only needs to be fast and slightly conservative, not exact (`TokenCounter::estimate_tokens` is explicitly documented as "faster, less precise" for exactly this kind of pre-flight use; `count_tokens`'s exact/slower path is not needed here).

## What this ADR explicitly does NOT solve

- **Does not fix the two-`ModelInfo`-types duplication.** `edge_llm_complete::ModelInfo` remains unrelated to `edge_llm_provider::ModelInfo`; this guard only ever reads the provider crate's own instance, since that's the only one `StdProvider` holds. Unifying the two types (or adding a conversion) is a separate, narrower cleanup, not bundled here.
- **Does not make the estimate exact.** `TokenCounter::estimate_tokens` is approximate by contract (`is_exact` may report `false` depending on implementation); this guard is a cheap local pre-check to avoid *most* wasted round-trips, not a guarantee that every vendor-side `ContextLengthExceeded` rejection disappears. The existing reactive mapping (`CompleteError::ContextLengthExceeded → ExecutionError::ContextWindowExceeded`) stays in place as the backstop for the residual gap between estimate and vendor tokenizer truth.
- **Does not address streaming mid-response overflow** (a completion that grows past the window across multiple streamed chunks) — this check is pre-flight only, evaluated once before dispatch.
- **Does not add a cost-ceiling, rate-limit, or output-length policy** — `CompositePolicy<ContextWindowCheckInput>` is deliberately left as a single-policy composite today; adding further gates of the *same input shape* is a natural follow-up this design enables but does not itself deliver.
- **Does not touch `edge-llm-complete`, `edge-llm-reasoning`, or `edge-llm-agent`** — the guard is scoped to `edge-llm-provider::StdProvider::complete()`, the one place `ModelInfo.context_window` and an assembled request are both in scope together.

## Consequences

**What this enables**
- Context-window overflows are caught locally, before any network call — no wasted round-trip (and, for paid vendor completers, no wasted spend) on a request already known to exceed the model's window.
- `ExecutionError::ContextWindowExceeded` finally has a proactive producer, not only a reactive one mapped from vendor rejections — the variant's full intent is now realized.
- Establishes the same `Policy`-based pattern ADR-046 established for tool governance as the house style for *any* future pre-flight numeric gate in `edge-llm-provider` (cost ceiling, output-length cap), with `CompositePolicy` already the composition point.
- `edge-llm-prompt`'s `TokenCounter` — defined, arch-compliant, and previously unreferenced anywhere — gets its first real consumer.

**What this requires**
- `provider/Cargo.toml`: add `edge-llm-prompt.workspace = true` to `[dependencies]`. Verified non-cyclic (see Decision).
- `StdProvider::new(...)` signature changes (breaking): two new required arguments, `token_counter: Arc<dyn TokenCounter>` and `context_policy: CompositePolicy<ContextWindowCheckInput>`. Every existing call site must be updated — at minimum `hello_edge.rs` (`swe-edge-bootstrap`'s example, per ADR-045) and this crate's own `#[cfg(test)]` `make_core` helper in `std_provider.rs`.
- New files: `api/provider/types/context_window_check_input.rs`, `core/provider/policies/context_window_policy.rs`, plus `saf/` re-export wiring for `ContextWindowPolicy` if it is to be constructible outside the crate (composition roots assembling `StdProvider` need to pass one in).
- A `TokenCounter` implementation must actually be supplied at every construction site — today only `TokenCounter`'s trait exists in `edge-llm-prompt`; whichever concrete impl(s) exist there (or a `Noop`/estimate-only default) becomes a required dependency for constructing any `StdProvider`, not optional.
- Test coverage per this codebase's mandatory `_happy`/`_error`/`_edge` scenarios for `ContextWindowPolicy::evaluate` (under budget / over budget / exactly at the boundary) and for `StdProvider::complete()`'s new proactive-rejection path (asserting no call reaches `self.completer` when the policy fails — e.g. via a spy/counting `Completer` test double).

## Alternatives Considered

**Inline the check directly in `StdProvider::complete()` (`if estimated + max_tokens_per_call > context_window`)**
Rejected. Since a `StdProvider::new` constructor change is unavoidable regardless (there is no existing `TokenCounter` seam to reuse), inlining saves no breaking-change cost while giving up independent testability, a uniform violation type (`PolicyError`), and the composability `CompositePolicy` already provides for free. It would also be the only cross-cutting rule in this crate family not expressed as a `Policy`, breaking the precedent ADR-046 just set.

**Make `context_policy`/`token_counter` optional (`Option<...>`, defaulting to a no-op pass-through) to avoid breaking `StdProvider::new`'s signature**
Rejected as the default shape, though noted as a softer migration path. An `Option` that silently defaults to "never check" would leave the exact same production risk this ADR exists to close for any call site that doesn't explicitly opt in — indistinguishable from not having the guard at all unless every construction site is separately audited. If migration friction proves too high in practice, a follow-up `StdProvider::with_context_guard(...)` builder-style method (additive, non-breaking) is the better relief valve than a defaulted-off `Option`.

**Add a brand-new `ContextWindowError`/bespoke gate type instead of reusing `Policy`/`ExecutionError::ContextWindowExceeded`**
Rejected for the same reason ADR-046 rejected rebuilding `GovernancePolicy`: `ExecutionError::ContextWindowExceeded` already exists with the right fields, and `Policy`/`PolicyError` already exists as the enforcement port. Inventing either would duplicate an existing, tested, arch-compliant concept for no behavioral gain.

**Push the check into `edge-llm-complete`'s `Completer` trait instead of `edge-llm-provider`**
Rejected. `Completer` never sees `ModelInfo`/`context_window` — that metadata belongs to `Provider`/`StdProvider`, one layer up. Pushing the check down would require threading `context_window` through every `Completer` implementation (including `EchoCompleter`/`NoopCompleter`), widening the change far beyond the one crate that actually has both pieces of information already in scope.

## Tracking

- `provider/Cargo.toml` — add `edge-llm-prompt` dependency
- `provider/main/src/api/provider/types/context_window_check_input.rs` — new type
- `provider/main/src/core/provider/policies/context_window_policy.rs` — new `Policy` impl
- `provider/main/src/api/provider/types/std_provider.rs` — new fields (`token_counter`, `context_policy`)
- `provider/main/src/core/provider/std/std_provider.rs` — `new()` signature change + `complete()` proactive check
- Call-site updates: `swe-edge-bootstrap`'s `hello_edge.rs` (per ADR-045), this crate's own test `make_core` helper
- Follow-up (not blocking): decide whether `edge-llm-runtime` (ADR-045) should supply a shared default `TokenCounter` impl for all registered providers, or require one per provider registration
- Follow-up (not blocking): softer, additive `StdProvider::with_context_guard(...)` migration path if the breaking constructor change proves too disruptive across existing call sites
- Not tracked here, separate cleanup: unify `edge_llm_provider::ModelInfo` and `edge_llm_complete::ModelInfo` (see "What this ADR explicitly does NOT solve")
