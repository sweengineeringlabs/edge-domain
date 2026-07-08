# ADR-072: Real ML/Vendor-Backed Moderation `Policy` — `edge-plugin-llm-moderation-openai`

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-042 (plugin boundary), ADR-046 (`edge-llm-tools` — origin of the "cross-cutting rule = `Policy` impl" pattern), ADR-048 (Real Vendor Completer — plugin shape + corrected `edge-security-runtime-credential` integration this ADR reuses verbatim), ADR-053 (Guardrails / `edge-llm-guardrails`, sibling — the exact contract this ADR plugs into)
**GitHub Issues:** TBD

---

## Context

ADR-053 built `edge-llm-guardrails` — a `GuardrailedCompleter` decorator wrapping `Arc<dyn Completer>`, enforcing a `CompositePolicy<GuardrailCheckRequest>` at `GuardrailPhase::PreCall` (before the inner completer runs) and `GuardrailPhase::PostCall` (after it returns). The only `Policy` impl ADR-053 designed was `DenylistPolicy` — keyword/regex matching. ADR-053 said this explicitly, and deferred the rest:

> "Real ML-based moderation (a vendor classifier or moderation endpoint) is explicitly a plugin-level concern per ADR-042's boundary — out of scope for this contract; a plugin providing one just plugs into the same `CompositePolicy` as another `Policy` impl, no contract change needed later."

Per explicit user direction, nothing in this audit series stays merely "deferred" — every gap ADR-053 (and its predecessors) named as a follow-up gets its own full ADR. This is that ADR for the moderation-backend deferral.

### The exact port this ADR must satisfy — verified against ADR-053, not re-derived

Read in full (`domain/scm/docs/adr/ADR-053-guardrails-content-moderation.md`) and cross-checked against the actual `edge-domain-policy` source (`domain/scm/domain-policy/main/src/api/policy/`):

```rust
// edge_domain_policy::Policy — domain-policy/main/src/api/policy/traits/policy.rs:13-24
pub trait Policy: Send + Sync {
    type Input;
    fn name(&self, req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError>;
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, Self::Input>) -> Result<(), PolicyError>;
}

// PolicyError — domain-policy/main/src/api/policy/errors/policy_error.rs:19-25
pub struct PolicyError { pub policy: &'static str, pub reason: String }

// PolicyEvaluateRequest — domain-policy/main/src/api/policy/types/policy_evaluate_request.rs:5-8
pub struct PolicyEvaluateRequest<'a, I> { pub input: &'a I }
```

And the exact `Input` shape a guardrail policy sees, per ADR-053 (`edge-llm-guardrails/api/types/`, not yet built but fully specified there):

```rust
pub struct GuardrailCheckRequest<'a> { pub text: &'a str, pub phase: GuardrailPhase }
pub enum GuardrailPhase { PreCall, PostCall }
```

So a moderation policy sees **flattened, plain text and nothing else** — no message roles, no conversation history, no caller identity (`GuardrailCheckRequest` carries no `SecurityContext`, per ADR-053's own named limitation), just `&str` plus which side of the call produced it. `CompositePolicy<GuardrailCheckRequest>` composes any number of `Policy<Input = GuardrailCheckRequest>` impls with AND semantics, evaluated in registration order, first `PolicyError` short-circuits the rest (`domain-policy/main/src/api/policy/types/composite_policy.rs:39-42`). This ADR designs one more entry to register into that same composite — nothing about `GuardrailCheckRequest`, `GuardrailPhase`, `CompositePolicy`, or `Policy` itself changes.

### Which "ML-based" shape, and why

Two readings of "ML-based moderation" were considered:

1. **A dedicated vendor moderation API** (analogous to OpenAI's `POST /v1/moderations`) — a stateless HTTP call, sending text, receiving categorized flags/scores back.
2. **A local, in-process classifier model** (e.g. an ONNX-hosted toxicity/safety classifier run inside the process).

Per ADR-042's plugin-boundary rule — binding on every LLM primitive in this workspace, restated verbatim in ADR-048 — "external network calls and vendor-specific parsing belong in a plugin, not domain code." Option 1 is a direct, structural match for that rule and for the precedent ADR-048 already set (`edge-plugin-llm-anthropic`: HTTP call + vendor JSON mapping, credential-resolved, living in a standalone plugin repo depending on the port crate, never the reverse). Option 2 additionally requires model-serving/runtime infrastructure (weight hosting, inference runtime, GPU/CPU capacity planning) that does not exist anywhere in this workspace today and is a separate, larger infrastructure decision, not a moderation-policy decision — building it here would smuggle a new subsystem into what should be a small plugin.

**Decision: Option 1.** OpenAI's `/v1/moderations` endpoint is used as the reference vendor API — chosen over Anthropic (ADR-048's first `Completer` vendor) specifically because Anthropic has no equivalent standalone, publicly documented moderation endpoint; OpenAI's is the direct off-the-shelf analog the prompt's own framing points at, returns categorized severity (`hate`, `harassment`, `self-harm`, `sexual`, `violence`, and sub-categories, each with a boolean `flagged` and a continuous `category_scores` value), and — as of this writing — is offered at no additional charge for text. That last fact is a vendor policy, not an architectural guarantee; see Consequences.

## Decision

Build a new standalone plugin repo, **`edge-plugin-llm-moderation-openai`**, implementing `edge_domain_policy::Policy<Input = GuardrailCheckRequest>` against OpenAI's `POST /v1/moderations`, following ADR-048's plugin shape exactly: depends on `edge-llm-guardrails` (for `GuardrailCheckRequest`/`GuardrailPhase`, the port types) and `edge-domain-policy` (for `Policy`/`PolicyError`/`CompositePolicy`) as ordinary library dependencies; neither of those crates gains a dependency back on this plugin.

### Credential resolution — reuse ADR-048's corrected pattern, not the stale copy

Same conclusion ADR-048 already reached and this ADR does not re-litigate: resolve the OpenAI API key via `edge-security-runtime-credential`'s `CredentialSourceResolver::resolve(CredentialSourceResolveRequest) -> Result<CredentialSourceResolveResponse, CredentialError>` (`security/runtime/scm/credential/main/src/api/traits/credential_source_resolver.rs:16-22`), obtained through `CredentialSourceResolverFactory::file()`. **Not** `transport/egress/http/scm/auth`'s `FileCredentialResolver` — that copy is pinned to the stale, pre-split `swe-edge-security` v0.3.3 tag ADR-048 already flagged as superseded. An API key is a plain bearer/header credential; no OAuth flow is needed.

### The policy itself

```rust
// core/openai_moderation_policy.rs — impl Policy<Input = GuardrailCheckRequest>
pub(crate) struct OpenAiModerationPolicy {
    config: ModerationConfig,
    http_egress: Arc<dyn HttpEgress>,
    credential_resolver: Arc<dyn CredentialSourceResolver>,
    observer: Arc<dyn ObserverContext>,
}

impl Policy for OpenAiModerationPolicy {
    type Input = GuardrailCheckRequest<'_>;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "openai-moderation" })
    }

    fn evaluate(&self, req: PolicyEvaluateRequest<'_, GuardrailCheckRequest<'_>>) -> Result<(), PolicyError> {
        // 1. resolve API key (CredentialSourceResolver)
        // 2. POST { "input": req.input.text } to /v1/moderations, Authorization: Bearer <key>
        // 3. parse { results: [{ flagged, categories: {..}, category_scores: {..} }] }
        // 4. record full categorized breakdown via self.observer (span attrs + counter),
        //    regardless of allow/deny outcome — see "PolicyError vs. richer type" below
        // 5. any category_scores[c] >= self.config.threshold_for(c) (or the API's own `flagged`
        //    when no override is configured) => Err(PolicyError::new("openai-moderation", reason))
        //    where `reason` is a compact, human-readable summary: e.g.
        //    "flagged categories: hate=0.91, violence=0.77 (phase=PostCall)"
        // 6. transport/parse failure => apply self.config.fail_mode (see Fail-mode below)
    }
}
```

Non-network fields worth calling out on `ModerationConfig` (`api/types/moderation_config.rs`): `base_url` (override for testing/self-hosted-compatible endpoints), `credential_source`, `category_thresholds: HashMap<&'static str, f64>` (defaults to the vendor's own `flagged` boolean when empty — an operator can tighten a specific category without forking the policy), `enabled_phases: Vec<GuardrailPhase>` (lets an operator run the paid check only `PreCall`, only `PostCall`, or both — see cost discussion below), and `fail_mode: ModerationFailMode { FailOpen, FailClosed }`.

### `PolicyError` vs. a richer categorized type — stays `PolicyError`, unchanged

`PolicyError { policy: &'static str, reason: String }` is a binary allow/deny signal by construction — `Policy::evaluate` returns `Result<(), PolicyError>`, and `CompositePolicy<I>` returns that same `Result<(), PolicyError>` verbatim from whichever policy first denies (`composite_policy.rs`'s doc example: `.evaluate(...).is_err()`). OpenAI's moderation response is strictly richer than that: multiple categories, each a continuous score, independent of whether any single one crosses a deny threshold.

Three options were weighed:

1. **Keep `PolicyError` exactly as-is**, pack a compact human summary into `reason` (top violated categories + scores), and separately emit the **full** categorized response (all categories, all scores, allow or deny) as `ObserverContext` span attributes + a counter (`moderation_checked_total{phase=..,flagged=..,category=..}`) at every evaluation, not just denials.
2. **Widen `PolicyError`** to add a `categories: Vec<CategoryScore>` field (or an `Option<serde_json::Value>` payload).
3. **Invent a new, moderation-specific error type** (`ModerationPolicyError`) parallel to `PolicyError`.

**Decision: option 1.** Options 2 and 3 were rejected, and deviating from the plain-`PolicyError` standard ADR-046/050/053 all converged on independently needs a real reason, not convenience — here is the real reason it doesn't clear that bar:

- `CompositePolicy<I>` is only useful because every `Policy<Input = I>` impl in a composite returns the *same* concrete error type. Widening `PolicyError` (option 2) is a breaking change to a type four other things already depend on exactly as-is (`DenylistPolicy`, ADR-046's `CapabilityGatePolicy`/`RiskCeilingPolicy`, ADR-050's `ContextWindowPolicy`) — every one of them would need to either populate a now-required richer field with nothing meaningful to put there, or the field would need to be optional, at which point it is not doing any real work for `PolicyError`'s existing callers.
- A parallel `ModerationPolicyError` (option 3) breaks composability outright: `CompositePolicy<GuardrailCheckRequest>`'s `Vec<Box<dyn Policy<Input = GuardrailCheckRequest>>>` requires every member to share one associated error path through `evaluate`'s `Result<(), PolicyError>` return type (the trait fixes the error type, not just the input type) — a second error type cannot sit in the same `Vec` without an adapter layer that re-erases it back down to `PolicyError` anyway, at which point the richer type never survives to a caller and bought nothing.
- The actual need behind "richer than allow/deny" is **audit/observability fidelity**, not **enforcement logic** — `GuardrailedCompleter`/`CompositePolicy` only ever need to know pass-or-fail to decide whether to proceed. ADR-053 already established the seam for "record more than the binary error carries": `ObserverContext`, wired at the exact same call site (`guardrail.decision` span attribute in ADR-053; extended here to also carry the moderation vendor's full per-category breakdown). Reusing that seam gets full fidelity to whoever reads traces/metrics without touching the enforcement type at all.

One additional real constraint drove this the rest of the way: flagged text should not be echoed verbatim into `reason` or into log/trace attributes — only category names and scores are recorded, never the offending text itself, to avoid propagating the very content being filtered into logs and traces that are typically retained and more broadly readable than the original request.

### Fail-mode: an explicit, configured choice — not a silent default

Unlike `DenylistPolicy` (a pure local computation that cannot itself fail), a network call to a vendor moderation endpoint can fail independently of the content being checked: timeout, 5xx, malformed JSON, rate-limit. `Policy::evaluate` has no dedicated space for "the check itself was inconclusive" — it is still forced through the same `Result<(), PolicyError>`. Hardcoding either direction is wrong for different reasons:

- **Hardcoded fail-closed** — any moderation-vendor outage denies **100% of completions system-wide**, an availability regression strictly worse than shipping with no ML moderation at all.
- **Hardcoded fail-open** — silently disables exactly the check during the moments (vendor incidents, network partitions) when behavior is least observed and most likely to matter, with no signal that it happened.

`ModerationConfig.fail_mode` makes this an explicit, deployment-time choice (`FailOpen`/`FailClosed`), and every transport/parse failure — success or fallback — is recorded via `ObserverContext` (`moderation_check_failed_total{fail_mode=..}`) so a fail-open occurrence is visible in metrics even though it does not block the request. No default is silently assumed; the composition root must set one.

### Addition, not replacement: `DenylistPolicy` stays, ordered first

`CompositePolicy` already supports registering more than one policy; this ADR adds a second entry, it does not remove ADR-053's first:

```rust
CompositePolicy::new()
    .with(Box::new(denylist_policy))          // cheap, local, zero network cost — checked first
    .with(Box::new(openai_moderation_policy))  // real backend, network call — checked second
```

Ordering matters, not just presence: `CompositePolicy` evaluates in registration order and short-circuits on first violation, so an obvious keyword hit is rejected by `DenylistPolicy` **before** any network call is made — the moderation API is only reached for content the free, local check already let through. This is genuine defense-in-depth cost avoidance, not just redundancy: the two checks catch different things (a denylist catches known-bad tokens instantly; a categorized classifier catches paraphrased/novel harmful content a keyword list would miss), and ordering the cheap one first means the expensive one is skipped exactly when it doesn't need to run.

### Shape / workspace layout

```
edge-plugin-llm-moderation-openai/                 (new standalone repo, mirrors edge-plugin-llm-anthropic's scm/ layout)
└── scm/
    └── main/src/
        ├── api/
        │   └── types/moderation_config.rs         (ModerationConfig: base_url, credential_source,
        │                                            category_thresholds, enabled_phases, fail_mode)
        ├── core/
        │   └── openai_moderation_policy.rs         (OpenAiModerationPolicy: impl Policy<Input=GuardrailCheckRequest>)
        ├── spi/
        │   ├── moderation_request.rs               (GuardrailCheckRequest -> OpenAI moderation JSON body)
        │   └── moderation_response.rs               (OpenAI JSON response -> allow/deny + categorized breakdown)
        └── saf/
            └── openai_moderation_policy_svc.rs      (openai_moderation_policy(config, http_egress,
                                                        credential_resolver, observer) -> impl Policy<Input=GuardrailCheckRequest>)
```

Depends on: `edge-llm-guardrails` (`GuardrailCheckRequest`, `GuardrailPhase` — port types only, per ADR-053), `edge-domain-policy` (`Policy`, `PolicyError`, `PolicyEvaluateRequest`, `PolicyNameRequest`/`Response`), `swe-edge-egress-http` (`HttpEgress`, for the moderation POST — no streaming/SSE needed, this is a single request/response call), `edge-security-runtime-credential` (`CredentialSourceResolver`, `CredentialSourceResolverFactory::file()`, at the current tag, not the stale `swe-edge-security` copy — see ADR-048), `edge-domain-observer` (`ObserverContext`). No dependency on `edge-llm-complete` or `edge-llm-provider` at all — this plugin only ever sees flattened text via `GuardrailCheckRequest`, never a `CompletionRequest`/`CompletionResponse`.

## What this ADR explicitly does NOT solve

- **A second/alternative moderation vendor** (Azure AI Content Safety, Google Perspective API, AWS Comprehend, or a local classifier per Option 2 above) — deliberately deferred as an independent, equally-cheap follow-on, exactly the "Anthropic first, OpenAI-compatible second" carve-out ADR-048 already used for `Completer` vendors. Nothing here blocks it; a second plugin implementing the same `Policy<Input = GuardrailCheckRequest>` port composes into the same `CompositePolicy` with zero contract change.
- **A local, in-process ML classifier** — considered (see Context) and rejected for v1 as a separate, larger infrastructure decision (model hosting/runtime), not a moderation-policy decision; not ruled out for a future ADR.
- **Per-tenant threshold or fail-mode variation.** `GuardrailCheckRequest` carries no `SecurityContext` (ADR-053's own named limitation, unchanged here) — `ModerationConfig` is a single, global configuration per `OpenAiModerationPolicy` instance. Tenant-aware variation is the same `Handler`-level decorator follow-up ADR-053 already named, not solved by this plugin.
- **Sampling or asynchronous best-effort post-call moderation** to reduce the cost/latency named below — named here as a real, available lever an operator could reach for later (e.g. only moderate post-call on a configurable sample rate, or moderate asynchronously and log-only rather than block an already-generated response) but not designed or built in this ADR; async/log-only post-call moderation would also directly contradict ADR-053's stated purpose for the post-call check (block content that must not reach the caller), so it is not a drop-in substitution, only a distinct, lesser guarantee an operator might knowingly choose.
- **Retry/backoff on moderation API 429/5xx** — the same inert-`RateLimited`/`NetworkError` gap ADR-048 already named for `AnthropicCompleter` recurs here; `fail_mode` governs what happens on a single failed attempt, nothing retries it.
- **Incremental (per-chunk) post-call moderation of a genuinely streamed response** — inherited unchanged from ADR-053; this plugin only ever receives a `GuardrailCheckRequest` at `PreCall` for `complete_stream`, same as `DenylistPolicy` does today.
- **Wiring `OpenAiModerationPolicy` into any live `CompositePolicy` in a composition root** (`swe-edge-bootstrap`, a future `edge-llm-runtime` wiring, or around `AnthropicCompleter`) — this ADR produces the plugin; registering it is separate follow-on work, the same carve-out ADR-053 already used for `GuardrailedCompleter` itself.
- **Guaranteeing OpenAI's moderation endpoint remains free of charge** — it is, as of this writing, offered at no additional charge for text; that is a vendor pricing decision this ADR has no control over and does not assume permanent (see Consequences).

## Consequences

**What this enables**
- The first *real*, categorized content-safety backend anywhere in `edge/` — closing ADR-053's own named gap, not just adding another keyword list.
- A concrete second data point (after `edge-plugin-llm-anthropic`, ADR-048) validating the plugin-boundary pattern for a non-`Completer` port: `Policy<Input = T>` is just as pluggable a seam for a vendor-backed implementation as `Completer` is.
- Full categorized moderation fidelity (all categories, all scores) visible in traces/metrics via `ObserverContext`, without widening `PolicyError` or breaking `CompositePolicy`'s uniform-error assumption for the three other `Policy` impls that already depend on it (`DenylistPolicy`, ADR-046's two tool-governance policies, ADR-050's `ContextWindowPolicy`).
- Configurable defense-in-depth: cheap local `DenylistPolicy` still runs first, unconditionally, at zero marginal cost, with the paid/networked check only reached for content the free one didn't already catch.

**What this requires — and the real operational cost to flag explicitly**
- New plugin repo `edge-plugin-llm-moderation-openai`, an `OPENAI_API_KEY` (or file-based credential) available at deployment time, and a composition root that actually registers it into `GuardrailedCompleter`'s `CompositePolicy` — none of this is automatic.
- **Latency: this genuinely triples the sequential network round-trips of a blocking `complete()` call, not doubles.** ADR-053's own design already runs the (free, local) `CompositePolicy` twice — once `PreCall`, once `PostCall` — around a single completion. Substituting or adding a real networked moderation check into *both* phases means: (1) a moderation API round-trip before the vendor completion call is even issued, (2) the vendor completion call itself, (3) a second moderation API round-trip after the completion returns and before the caller sees it — three sequential network hops where there was previously one. For `complete_stream`, the `PreCall` check still adds a full moderation-API round-trip *before the first output token can begin streaming* — a direct, real cost to a feature (streaming) whose entire value proposition is fast time-to-first-token; this is a tradeoff an operator should be aware of, not a free win.
- **Cost: this ADR assumes OpenAI's `/v1/moderations` endpoint stays free-of-charge for text, and that assumption is not architecturally guaranteed.** If pricing changes, or a different vendor/endpoint is substituted (per the deferred multi-vendor follow-up above) that does charge per call, the operational cost is two full moderation-API calls per blocking completion (pre + post) — a direct multiplier on request volume, distinct from and additive to whatever the underlying `Completer` vendor already charges for the completion itself.
- `enabled_phases` and `fail_mode` in `ModerationConfig` exist precisely so an operator can trade coverage for latency/cost/availability deliberately (e.g. `PreCall`-only to avoid doubling round-trips, or `FailOpen` to avoid an outage becoming a full outage) — but someone must actually choose those values; shipping with unconsidered defaults would silently pick a tradeoff no one reviewed.
- No changes to `edge_domain_policy`'s `Policy`/`PolicyError`/`CompositePolicy`, no changes to `edge-llm-guardrails`'s `GuardrailCheckRequest`/`GuardrailPhase`/`GuardrailedCompleter` — this plugin is purely additive at the same seam ADR-053 already left open for it.

## Alternatives Considered

**A local, in-process ML classifier instead of a vendor moderation API**
Rejected for v1. Requires model-serving/runtime infrastructure (weight distribution, inference runtime, CPU/GPU capacity planning) that exists nowhere in this workspace today — a separate infrastructure decision, not a `Policy`-shape decision. A stateless HTTP call to an existing, already-categorized, already-maintained vendor endpoint is the lower-risk, structurally consistent first step (mirrors ADR-048's own reasoning for choosing a real vendor `Completer` over building one from scratch). Not rejected as *never useful* — named explicitly as a viable future ADR once/if data-residency, offline, or cost requirements make an in-process model the better tradeoff.

**Widening `PolicyError` with a `categories: Vec<CategoryScore>` field, or a distinct `ModerationPolicyError` type**
Rejected. See "`PolicyError` vs. a richer categorized type" above — both break `CompositePolicy<I>`'s uniform-error-type assumption that three other already-designed `Policy` impls (`DenylistPolicy`, `CapabilityGatePolicy`/`RiskCeilingPolicy` from ADR-046, `ContextWindowPolicy` from ADR-050) already rely on, for a fidelity need (audit/observability) the existing `ObserverContext` seam already satisfies without touching the enforcement type.

**Replacing `DenylistPolicy` with the real backend rather than composing both**
Rejected. The two catch different things (exact-match tokens vs. classified/paraphrased content) at very different costs (zero vs. a network call), and `CompositePolicy`'s ordered, short-circuiting composition means keeping both costs nothing when the cheap one is checked first. Removing the free check to "simplify" would only add latency/cost with no coverage gain.

**Hardcoding a single fail-mode (always fail-open, or always fail-closed) instead of making it configurable**
Rejected. Each hardcoded choice is wrong for a different real deployment: fail-closed turns a vendor outage into a total outage of every LLM completion; fail-open silently disables the exact protection during an incident. Neither is a universally correct default across every deployment this ADR's plugin could be used in, so the choice is made an explicit, observable, per-deployment configuration value instead of an assumption baked into the code.

**Running the real backend only `PostCall` (or only `PreCall`) to halve the network cost, hardcoded into the plugin**
Rejected as a hardcoded default, kept as a configuration option (`enabled_phases`). ADR-053 already justified running both phases for the local `DenylistPolicy` ("deferring either one would leave a caller with no protection in that direction... for no implementation savings") — that reasoning applies identically to a real backend's *coverage*, so this ADR does not silently narrow it. But unlike the local check, the real backend's *cost* is non-trivial, so — unlike `DenylistPolicy` — this plugin exposes the choice as configuration rather than deciding it in code, so an operator can deliberately trade coverage for cost/latency with the tradeoff named, not hidden.

## Tracking

- New repo: `sweengineeringlabs/edge-plugin-llm-moderation-openai`
- Depends on (already proposed, not yet built): ADR-053's `edge-llm-guardrails` crate (`GuardrailCheckRequest`, `GuardrailPhase`, `CompositePolicy<GuardrailCheckRequest>`) must exist before this plugin can compile against it
- Follow-up (independent, not blocking): a second/alternative moderation vendor plugin (Azure Content Safety, Google Perspective API, or others) — same shape, different vendor JSON mapping and auth header, mirroring ADR-048's Anthropic→OpenAI-compatible carve-out
- Follow-up (separate, larger ADR): a local, in-process ML classifier `Policy` impl, if data-residency/offline/cost requirements later favor it over a vendor API call
- Follow-up (not blocking): sampling or asynchronous best-effort post-call moderation as a cost/latency lever, with the caveat named above that it is a lesser guarantee, not a transparent substitution
- Follow-up (separate ADR, named as an explicit gap above, same as ADR-053's own carve-out): tenant-aware policy/threshold variation via a `SecurityContext`-carrying `Handler`-level decorator
- Follow-up (separate ADR/issue, mirrors ADR-048's own tracking): register `OpenAiModerationPolicy` into a live `CompositePolicy` behind `GuardrailedCompleter` in a real composition root
- Not blocking this ADR: retry/backoff consumption of moderation-API rate-limit/network failures (same inert-gap pattern ADR-048 already named for `AnthropicCompleter`)
