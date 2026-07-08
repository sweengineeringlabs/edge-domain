# ADR-051: Retry/Backoff Wiring for LLM Handler Execution

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-045 (`edge-llm-runtime` — same decorator pattern as `TimeoutHandler`), ADR-046 (`edge-llm-tools` — sibling `Handler`-wrapping decorator, `GovernedHandler`), ADR-047 (`edge-runtime-*` primitive family — the contract-crate pattern this ADR extends)
**GitHub Issues:** TBD

---

## Context

`ExecutionError` (`domain/scm/domain/llm/provider/main/src/core/provider/execution/execution_error.rs`) has carried `is_retryable()` and `retry_after()` since it was written:

```rust
pub fn is_retryable(&self) -> bool {
    matches!(
        self,
        ExecutionError::RateLimited { .. }
            | ExecutionError::ProviderUnavailable { .. }
            | ExecutionError::NetworkError(_)
            | ExecutionError::Timeout { .. }
            | ExecutionError::QuotaExceeded { .. }
    )
}

pub fn retry_after(&self) -> Option<Duration> { /* ... */ }
```

The 2026-07-08 landscape audit confirmed both methods are consumed **only by their own unit tests in the same file** — grepping the whole `domain/scm/domain/llm` tree finds no call site, no retry loop, no backoff sleep, anywhere. Worse: the one place an `ExecutionError`-shaped failure actually crosses a `Handler` boundary today, `default_provider_handler.rs`, already throws the structure away:

```rust
.map_err(|e| HandlerError::ExecutionFailed(e.message()));
```

`HandlerError` (`edge-domain-handler`) has no variant that preserves retryability or a retry-after hint — `e.message()` stringifies it. So even a caller that wanted to retry today has nothing to retry *on* once execution has returned through the `Handler` port. This is a second, previously-unnoticed half of the same gap: the dead code isn't only "no retry loop" — it's "no retry loop, and the one existing error-mapping site actively discards the signal a retry loop would need."

Meanwhile, a mature retry/backoff implementation already exists in this monorepo: `transport/egress/grpc/scm/retry` (`swe-edge-egress-grpc-retry`). Per ADR-045's lesson — checking the aliased/wrong crate and wrongly concluding "unused" — this ADR verified the crate directly rather than assuming it's reusable because it "looks generic" from file names alone (`api/retry/backoff/backoff_scheduler.rs` reads as a plausible transport-agnostic primitive).

It is not. Four separate couplings to gRPC were found by reading the crate's actual contents:

1. **Cargo dependency.** `Cargo.toml`: `swe-edge-egress-grpc = { workspace = true }` — a direct, unaliased dependency on the gRPC transport crate itself (`GrpcEgress`, `GrpcChannelConfig`, `TransportConstruction`). Not a false-alarm alias this time — the coupling is real and undisguised.
2. **The trait signature is hardwired to `GrpcRetryConfig`.** `api::retry::traits::backoff_scheduler::BackoffScheduler::schedule(&self, req: BackoffScheduleRequest) -> Result<ScheduleResponse, Error>`, where `BackoffScheduleRequest { config: GrpcRetryConfig, attempt: u32, random_unit: f64, track: BackoffTrack }`. There is no generic policy parameter — a non-gRPC caller cannot implement or call this trait without constructing a `GrpcRetryConfig`.
3. **Retryability classification is gRPC-status-specific, by design.** `api::retry::types::retry_decision.rs`'s doc comment is explicit: `Unavailable`/`ResourceExhausted` retryable, `Unauthenticated`/`PermissionDenied`/`DeadlineExceeded`/`Internal` not — reasoning entirely about tonic status-code semantics. This is a *different* decision surface than `ExecutionError::is_retryable()` (`RateLimited`/`ProviderUnavailable`/`NetworkError`/`Timeout`/`QuotaExceeded`), which has no gRPC status codes at all. Reusing `RetryDecision` for LLM would be reusing the wrong classification, not just an inconvenient one.
4. **The decorator target is `GrpcEgress`, not `Handler`.** `GrpcRetryClient<T>` (`core/retry/grpc/grpc_retry_client.rs`) wraps `T` and implements `GrpcEgress` for the wrapper (per `resilient/main/src/core/traits/grpc_resilient_facade.rs`: `GrpcRetryClient::new(base, retry_cfg)` composes directly with `GrpcBreakerClient`). It decorates a transport trait, not the domain `Handler<Req, Resp>` port every LLM crate is built on.

The one genuinely transport-agnostic piece — `core::retry::backoff::backoff_scheduler::BackoffScheduler::exponential_jitter` (pure `Duration`/`f64`/attempt-index math, no gRPC types anywhere in its signature) — is `pub(crate)`. It isn't exported. A non-gRPC caller can't import it; it can only be copy-pasted.

`edge-dispatcher`'s `TimeoutHandler<H>` (`dispatcher/scm/main/src/core/handler/timeout_handler.rs`, `api/handler/types/timeout_handler.rs`), surfaced during ADR-045's research, remains the right shape to imitate: a plain `impl<H: Handler> Handler for TimeoutHandler<H>` struct holding `inner: H` plus policy fields, wrapping `self.inner.execute(...)` with a cross-cutting concern and translating failure into a `HandlerError` variant. ADR-046's `GovernedHandler<H>` already applied this exact pattern for tool governance. Retry is the third instance of the same shape.

## Decision

**Extract a transport-agnostic backoff contract crate, `edge-runtime-retry`, into the `edge-runtime-*` family (ADR-047), and build `RetryHandler<H>` in `edge-llm-provider` against it** — not a direct dependency on `swe-edge-egress-grpc-retry`.

This is forced by the Context findings, not a stylistic preference: the existing crate's only reusable part (`exponential_jitter`) is private, and its public surface (trait, config type, retryability classification, decorator target) is gRPC-specific in all four ways enumerated above. Depending on `swe-edge-egress-grpc-retry` directly from an LLM domain crate would also invert the SEA dependency direction — a `domain/` primitive taking a hard dependency on a `transport/egress/` crate, which no other `edge-domain-*`/`edge-llm-*` crate does today (`edge-domain-handler`'s own `HandlerError` re-export is the only cross-tier import point, and it's one-directional the other way: transport depends on domain, never the reverse).

`edge-runtime-retry` follows ADR-047 §1 exactly: **contracts only** — `api/traits/`, `api/types/`, `saf/`, no `core/`, no transport dependencies (no tonic, no Axum, no AMQP). It defines:

- `trait BackoffPolicy: Send + Sync { fn next_delay(&self, req: BackoffRequest) -> Result<BackoffResponse, BackoffError>; }` — generalizes `exponential_jitter`'s parameters (`initial`, `max`, `multiplier`, `jitter_factor`, `attempt`, `random_unit`) as plain `Duration`/`f64`/`u32` fields on `BackoffRequest`, with no `GrpcRetryConfig`, no gRPC status codes anywhere.
- `BackoffRequest { initial: Duration, max: Duration, multiplier: f64, jitter_factor: f64, attempt: u32, random_unit: f64 }` / `BackoffResponse { sleep: Duration }` — the Request/Response port shape, mandatory per arch 0.2.46.
- `BackoffError` (`*Error`-suffixed) — for malformed policy parameters (e.g. `multiplier <= 0.0`), not for the underlying call's own failure (that stays `HandlerError`/`ExecutionError`, unchanged).
- **No retryability classification.** Whether a given failure is worth retrying stays fully local to each caller — gRPC keeps `RetryDecision` against tonic status codes; LLM keeps `ExecutionError::is_retryable()`/`retry_after()` against provider-shaped errors. Only the scheduling *arithmetic* (exponential backoff + jitter + cap) is shared. This mirrors ADR-046's line between what's genuinely shared (`Policy`, `SecurityContext`, `ObserverContext`) and what stays domain-specific (`CapabilityFlags`, `RiskLevel`) — retry classification is domain-specific in exactly the same way.

### Shape / workspace layout

```
runtime/scm/retry/                       (edge-runtime-retry — NEW, ADR-047 family)
├── api/
│   ├── traits/backoff_policy.rs         (BackoffPolicy)
│   ├── types/{backoff_request,backoff_response}.rs
│   └── errors/backoff_error.rs          (BackoffError)
└── saf/
    └── backoff_policy_svc.rs            (re-export only)

domain/scm/domain/llm/provider/main/src/
├── core/provider/
│   └── retry_handler.rs                 (NEW — RetryHandler<H>, mirrors timeout_handler.rs)
└── api/provider/types/
    └── retry_policy.rs                  (NEW — RetryPolicy: max_attempts, initial, max, multiplier, jitter_factor)

domain/scm/domain-handler/main/src/api/handler/errors/
└── handler_error.rs                     (MODIFIED — new Retryable variant, see below)
```

`RetryHandler<H>` is the same shape as `TimeoutHandler<H>`:

```rust
pub struct RetryHandler<H> {
    pub(crate) inner: H,
    pub(crate) policy: RetryPolicy,               // max_attempts, initial, max, multiplier, jitter_factor
    pub(crate) scheduler: Arc<dyn BackoffPolicy>, // edge-runtime-retry, injected — no concrete impl owned here
}

#[async_trait]
impl<H> Handler for RetryHandler<H>
where
    H: Handler,
    H::Request: Clone + Send + 'static,
    H::Response: Send + 'static,
{
    type Request = H::Request;
    type Response = H::Response;

    fn id(&self, req: IdRequest) -> Result<IdResponse, HandlerError> { self.inner.id(req) }
    fn pattern(&self, req: PatternRequest) -> Result<PatternResponse, HandlerError> { self.inner.pattern(req) }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, H::Request>,
    ) -> Result<H::Response, HandlerError> {
        let mut attempt: u32 = 0;
        loop {
            let retry_req = ExecutionRequest { req: req.req.clone(), ctx: req.ctx };
            match self.inner.execute(retry_req).await {
                Ok(resp) => return Ok(resp),
                Err(HandlerError::Retryable { retry_after, reason })
                    if attempt < self.policy.max_attempts =>
                {
                    let sleep = retry_after.unwrap_or_else(|| {
                        self.scheduler
                            .next_delay(self.policy.as_backoff_request(attempt, jitter_sample()))
                            .map(|r| r.sleep)
                            .unwrap_or(self.policy.initial)
                    });
                    tokio::time::sleep(sleep).await;
                    attempt += 1;
                    tracing::debug!(attempt, reason, "retrying handler execution");
                }
                Err(e) => return Err(e), // non-retryable, or attempts exhausted
            }
        }
    }
}
```

Bounded by construction: the loop only continues while `attempt < self.policy.max_attempts` (a required, finite `RetryPolicy` field — never `None`/unbounded) — satisfying this repo's CLAUDE.md mandate against unbounded retries. `H::Request: Clone` is the one new constraint on wrapped handlers; every registration-time `Handler` in the LLM crates today (`String`, `ExecutionStepResult`'s request types) is a plain, cheaply-cloned value type, so this is not a practical restriction.

**Required companion change — `HandlerError::Retryable`.** `RetryHandler<H>` cannot work against today's `HandlerError` because nothing preserves retry structure across the `Handler` boundary (see Context: `default_provider_handler.rs`'s `HandlerError::ExecutionFailed(e.message())`). This ADR adds one variant to `edge-domain-handler`'s `HandlerError`:

```rust
/// The operation failed but may succeed on retry.
#[error("retryable failure: {reason}")]
Retryable { retry_after: Option<Duration>, reason: String },
```

and changes the one existing mapping site, `default_provider_handler.rs`'s `execute()`, from unconditionally stringifying `ExecutionError` into `ExecutionFailed` to routing through `is_retryable()`/`retry_after()` first:

```rust
.map_err(|e| {
    if e.is_retryable() {
        HandlerError::Retryable { retry_after: e.retry_after(), reason: e.message() }
    } else {
        HandlerError::ExecutionFailed(e.message())
    }
});
```

This is a shared-crate change (`edge-domain-handler`, consumed by every `Handler` impl in the monorepo, not just LLM), so it is called out explicitly as its own line item in Consequences, not folded silently into "add a decorator."

## What this ADR explicitly does NOT solve

- **No real vendor `Completer` exists to retry against.** Same caveat as ADR-045: today's only `Completer`s are `EchoCompleter`/`NoopCompleter`, which don't fail in the ways `ExecutionError::RateLimited`/`ProviderUnavailable`/`NetworkError` model. `RetryHandler<H>` proves the *plumbing* — retryable failure in, bounded backoff, re-invocation — the same way ADR-045 proved HTTP transport plumbing against an echo backend. Exercising it against a real rate-limited vendor API is separate follow-on work.
- **No circuit-breaking.** `swe-edge-egress-grpc-breaker` already owns that concern for gRPC egress; this ADR does not propose an LLM equivalent. If sustained provider failures need a breaker, that's a distinct ADR.
- **No idempotency handling for partially-streamed responses.** `RetryHandler<H>` wraps `Handler::execute` (buffered, single request/response). `StreamHandler`/`buffered_stream_handler.rs` — where a retry after a mid-stream failure could mean re-emitting already-delivered tokens to a caller — is out of scope; streaming retry semantics need their own design.
- **Does not migrate `swe-edge-egress-grpc-retry` onto `edge-runtime-retry`.** The gRPC crate's private `exponential_jitter` and the new crate's `BackoffPolicy` compute the same math; unifying them (so there is exactly one tested implementation of exponential-jitter-with-cap in the monorepo, not two) is a natural, non-blocking follow-up — additive and non-breaking per ADR-047's own migration pattern (§5), not required for this ADR's first cut.
- **Does not wire `RetryHandler` into `edge-llm-runtime` (ADR-045) registration by default.** Whether the provider handler is registered raw or wrapped in `RetryHandler` is an explicit composition-root choice, the same opt-in posture ADR-046 took for `GovernedHandler` — an unwrapped registration is still possible and won't be flagged by tooling.
- **Does not address context-window overflow, tool governance, or any other gap from the 2026-07-08 landscape audit** — those remain tracked under ADR-046 and the audit's own backlog, unaffected by this ADR.

## Consequences

**What this enables**
- `ExecutionError::is_retryable()`/`retry_after()` stop being dead code — a concrete, bounded retry loop consumes them for the first time.
- A single, tested, reusable backoff primitive (`edge-runtime-retry::BackoffPolicy`) usable by any future non-gRPC egress that needs exponential-jitter scheduling, without needing to depend on gRPC-specific types to get it — closing the same kind of contract-crate gap ADR-047 closed for HTTP.
- `RetryHandler<H>` composes with the LLM crates' existing `Handler` port exactly like `TimeoutHandler<H>` and `GovernedHandler<H>` already do — a composition root can stack `RetryHandler<GovernedHandler<TimeoutHandler<DefaultProviderHandler>>>` (or any order) using only existing, proven decorator machinery, no new registration mechanism.
- `HandlerError::Retryable` gives every `Handler` in the monorepo — not just LLM — a structured way to signal "retryable" for the first time; other domains (not just LLM) can adopt `RetryHandler<H>` against their own handlers without inventing their own retry-signal variant.

**What this requires**
- New crate `edge-runtime-retry` (`runtime/scm/retry/`), contracts-only per ADR-047 §1 — no `core/`.
- New file `domain/scm/domain/llm/provider/main/src/core/provider/retry_handler.rs` (+ `api/provider/types/retry_policy.rs`) implementing `RetryHandler<H>`.
- **A modification to `edge-domain-handler`'s `HandlerError`** (new `Retryable` variant) and to `default_provider_handler.rs`'s one existing error-mapping call site — a shared-crate change, flagged explicitly rather than bundled silently into "add a decorator" (per this repo's CLAUDE.md: no declare-and-abandon, no silent scope creep).
- `H::Request: Clone` becomes a bound on any handler wrapped in `RetryHandler<H>` — satisfied trivially by every LLM `Handler`'s request type today, but a real constraint future handler authors must keep in mind.
- No changes to `swe-edge-egress-grpc-retry`, `swe-edge-egress-grpc-resilient`, or gRPC's own retry classification — both stay exactly as they are.

## Alternatives Considered

**Depend directly on `swe-edge-egress-grpc-retry` from `edge-llm-provider`**
Rejected. Per Context, the crate's public surface is gRPC-specific in four independent ways (Cargo dependency on the transport crate, `GrpcRetryConfig`-typed trait signature, gRPC-status-specific `RetryDecision`, `GrpcEgress`-typed decorator target), and its one transport-agnostic piece (`exponential_jitter`) is `pub(crate)` and unreachable. Taking this dependency would also invert SEA's dependency direction — a `domain/` crate depending on a `transport/egress/` crate — which no other LLM or domain crate does today.

**Copy the exponential-jitter arithmetic directly into `edge-llm-provider`, no shared crate**
Rejected. Produces two independently-maintained, untested-against-each-other implementations of the same exponential-backoff-with-jitter-and-cap math, with no shared contract and no way for arch-audit or any future consumer to know they're supposed to agree. Directly contradicts the "single source of truth" reasoning ADR-046 used to reject reinventing `Policy`/`SecurityContext`/`AuditEntry`.

**Put the retry loop inside `Provider::complete()` / `ExecutionModel::execute_step()` directly, not a `Handler` decorator**
Rejected for the same reason ADR-046 rejected baking governance into `Skill`/`Agent`: it couples a cross-cutting, purely operational concern (how many times to re-attempt a call, how long to wait) to domain logic, requires every `ExecutionModel`/`Completer` implementer to reimplement it, and doesn't compose uniformly across provider/prompt/reasoning/agent handlers the way one `RetryHandler<H>` wrapping any of them does.

**Leave `HandlerError` unchanged; have `RetryHandler<H>` string-match on `HandlerError::ExecutionFailed`'s message to guess retryability**
Rejected outright — stringly-typed control flow over an error message is exactly the anti-pattern this repo's CLAUDE.md names (`if status == "active"`-style stringly typing) and would silently break the moment any error message's wording changed. The structured `Retryable` variant is the only sound option.

## Tracking

- New crate: `edge-runtime-retry` (`runtime/scm/retry/`) — next `edge-runtime-*` family member per ADR-047, contracts-only
- New file: `domain/scm/domain/llm/provider/main/src/core/provider/retry_handler.rs` (+ `retry_policy.rs`)
- Cross-cutting change: `edge-domain-handler`'s `HandlerError` gains `Retryable { retry_after: Option<Duration>, reason: String }`; `default_provider_handler.rs`'s mapping site updated to populate it
- Follow-up (non-blocking): migrate `swe-edge-egress-grpc-retry`'s private `exponential_jitter` onto `edge-runtime-retry::BackoffPolicy`, so there is exactly one tested implementation in the monorepo
- Follow-up (non-blocking): decide, alongside ADR-045's own open item, whether `edge-llm-runtime` wraps the provider handler in `RetryHandler` (and in what stacking order relative to `TimeoutHandler`/`GovernedHandler`) by default, or requires opt-in per handler
- Depends on: ADR-047 (contract-crate pattern), ADR-045/046 (decorator precedent)
