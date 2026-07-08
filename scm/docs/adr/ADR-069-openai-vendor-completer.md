# ADR-069: Real Vendor `Completer` — OpenAI Chat Completions API as Second Backend

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-043 (LLM Complete), ADR-048 (Real Vendor Completer — Anthropic, sibling plugin), [edge ADR-042](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-042-llmprovider-reshape-to-edge-plugin.md) (plugin boundary), ADR-045 (edge-llm-runtime), ADR-015 (Security Layer / credential resolution)
**GitHub Issues:** TBD

---

## Context

ADR-048 built `edge-plugin-llm-anthropic`, the first real (non-echo, non-noop) implementation of `edge_llm_complete::Completer`, and — deliberately, per its own "Why Anthropic first" section — deferred the second vendor rather than building both in one pass: *"An OpenAI-compatible (`/v1/chat/completions`) implementation is an equally cheap, structurally identical follow-on — same trait, same SSE transport trait, different JSON shape and a different vendor auth header."* ADR-048's own Tracking section lists this explicitly: *"Follow-up (independent, not blocking): `edge-plugin-llm-openai` — same shape, different vendor JSON mapping and auth header."*

Per explicit user direction, nothing in this audit chain stays merely deferred — every follow-up ADR-048 named gets its own full ADR rather than a one-line tracking bullet. This is that ADR.

Nothing about the case for a second vendor has changed since ADR-048 was written: `Completer`, `CompletionRequest`/`CompletionResponse`, and `StreamChunk`/`StreamDelta` are all vendor-agnostic (ADR-048, Context and Decision). This ADR does the work ADR-048 gestured at: it actually maps OpenAI's Chat Completions API onto the same port, the same streaming types, and the same plugin-boundary rule — and it checks, rather than assumes, whether any of ADR-048's `spi/` code is reusable here.

### The exact port to satisfy (unchanged from ADR-048)

`domain/scm/domain/llm/complete/main/src/api/complete/traits/completer.rs:18-83` — re-read directly for this ADR, not taken on ADR-048's word:

```rust
#[async_trait]
pub trait Completer: Send + Sync {
    async fn complete(&self, req: CompleteRequest<'_>) -> Result<CompletionResponse, CompleteError>;
    async fn complete_stream(&self, req: CompletionStreamRequest<'_>) -> Result<CompletionStreamResponse, CompleteError>;
    fn supported_models(&self, req: SupportedModelsRequest) -> Result<SupportedModelsResponse, CompleteError>;
    fn supports(&self, req: ModelSupportRequest<'_>) -> Result<ModelSupportResponse, CompleteError> { /* default */ }
    async fn model_info(&self, req: ModelInfoRequest<'_>) -> Result<ModelInfoResponse, CompleteError>;
    async fn list_models(&self, req: ListModelsRequest) -> Result<ListModelsResponse, CompleteError>;
    async fn is_model_available(&self, req: ModelAvailabilityRequest<'_>) -> Result<ModelAvailabilityResponse, CompleteError> { /* default */ }
    async fn health_check(&self, _req: CompleterHealthCheckRequest) -> Result<CompleterHealthCheckResponse, CompleteError> { /* default */ }
}
```

Identical to what ADR-048 satisfied: five methods to implement (`complete`, `complete_stream`, `supported_models`, `model_info`, `list_models`); `supports`, `is_model_available`, `health_check` come free via default trail-through. `CompleteError` (`complete_error.rs:1-81`, 15 variants) already covers every failure mode this plugin needs — `NetworkError`, `StreamError`, `AuthenticationFailed`, `RateLimited { retry_after_ms }`, `ProviderError { provider, message }`, `ModelNotFound` — confirmed by direct read, not by reuse of ADR-048's claim. **Zero new error variants required, exactly as ADR-048 found for Anthropic.**

### Credential resolution — same current mechanism ADR-048 corrected onto

ADR-048 was corrected mid-session away from the stale `transport/egress/http/scm/auth` / `swe-edge-security` v0.3.3 copy, onto the current, post-split family: `edge-security-runtime-credential` (`security/runtime/scm/credential/`), specifically:

```rust
// security/runtime/scm/credential/main/src/api/traits/credential_source_resolver.rs:16-22
pub trait CredentialSourceResolver: Send + Sync {
    fn resolve(&self, req: CredentialSourceResolveRequest) -> Result<CredentialSourceResolveResponse, CredentialError>;
}
```

This ADR inherits that corrected integration point unchanged — it does **not** re-derive credential resolution, and it does **not** regress onto the stale `swe-edge-security` v0.3.3 copy either. `FileCredentialSourceResolver` (`spi/file_credential_source_resolver.rs`) via `CredentialSourceResolverFactory::file() -> Box<dyn CredentialSourceResolver>` is reused as-is. An OpenAI API key is, like Anthropic's, a plain bearer credential — no `OAuthTokenSourceFactory` needed for v1. The only vendor-specific difference is *which HTTP header* the resolved credential is written into (see below) — the resolution mechanism itself is identical and already proven by ADR-048.

### Real SSE transport — same trait, different framing discipline

`transport/egress/http/scm/transport/main/src/api/traits/http/http_stream.rs:20-31`'s `HttpStream::subscribe_sse` / `SseStream` / `SseEvent { event: Option<String>, data: String, id: Option<String> }` is reused unchanged — it already parses generic `text/event-stream` line framing (`event:`/`data:`/`id:` fields), regardless of vendor. What differs vendor-to-vendor is *what's inside* `SseEvent.data` and *whether `SseEvent.event` is populated at all* — see Streaming Path below; OpenAI's protocol does not use named SSE events the way Anthropic's does.

## Decision

Build a new standalone plugin repo, **`edge-plugin-llm-openai`**, implementing `edge_llm_complete::Completer` against OpenAI's Chat Completions API (`POST /v1/chat/completions`), following ADR-042's plugin-boundary rule exactly — identical to ADR-048's Anthropic plugin in every structural respect: depends on `edge-llm-complete` (port contracts), `swe-edge-egress-http` (HTTP/SSE transport), and `edge-security-runtime-credential` (credential resolution) as ordinary library dependencies; none of those three crates gains a dependency back on this plugin.

### Shape / workspace layout

```
edge-plugin-llm-openai/                         (new standalone repo, mirrors edge-plugin-llm-anthropic's scm/ layout)
└── scm/
    └── main/src/
        ├── api/
        │   └── types/openai_config.rs           (OpenAiConfig: model defaults, base_url override, credential_source)
        ├── core/
        │   └── openai_completer.rs               (OpenAiCompleter: impl Completer)
        ├── spi/
        │   ├── openai_request.rs                 (CompletionRequest → OpenAI chat/completions JSON body)
        │   ├── openai_response.rs                 (OpenAI JSON response → CompletionResponse)
        │   └── openai_stream.rs                   (SseEvent → StreamChunk/StreamDelta incremental parser)
        └── saf/
            └── openai_completer_svc.rs            (openai_completer(config, http_stream, http_egress) -> impl Completer)
```

This is a file-for-file mirror of ADR-048's layout (`api/types/*_config.rs`, `core/*_completer.rs`, `spi/*_request.rs`+`*_response.rs`+`*_stream.rs`, `saf/*_completer_svc.rs`) — same SEA shape, same naming convention, only the vendor token (`anthropic` → `openai`) changes.

Depends on: `edge-llm-complete` (`Completer`, `CompleteError`, `CompletionRequest`/`Response`, `StreamChunk`/`StreamDelta`, `ToolCallDelta`), `swe-edge-egress-http` (`HttpEgress`, `HttpStream`, `SseStream`, `SseEvent`), `edge-security-runtime-credential` (`CredentialSourceResolver`, `CredentialSourceResolveRequest`/`Response`, `CredentialSourceResolverFactory::file()`) — same current tag as ADR-048, not the stale `swe-edge-security` v0.3.3.

### `OpenAiCompleter` — mapping each trait method

- **`complete`** — builds an OpenAI `chat/completions` request body from `CompletionRequest.messages`. Role mapping: `Role::User`→`"user"`, `Role::Assistant`→`"assistant"`, `Role::System`→`"system"`, `Role::Tool`→`"tool"` — **all four stay as entries in the `messages` array**; unlike Anthropic, OpenAI has no top-level `system` field to hoist onto, so `openai_request.rs`'s role mapping is a straight per-message translation with no request-shape restructuring. Auth is a `Authorization: Bearer {api_key}` header (not Anthropic's `x-api-key`) carrying the credential resolved via `CredentialSourceResolver`. The JSON response (`choices[0].message.content`, `choices[0].finish_reason`, `usage.prompt_tokens`/`usage.completion_tokens`) maps into `CompletionResponse`. Error mapping mirrors ADR-048's table structurally (non-2xx/malformed JSON → `CompleteError::ProviderError { provider: "openai", message }`; connection failure → `NetworkError`; 401 → `AuthenticationFailed`; 429 → `RateLimited { retry_after_ms }` read from OpenAI's `retry-after` header when present) — the *shape* of this mapping is identical to Anthropic's, only the provider tag and the header name that produced a 401 differ.
- **`complete_stream`** — calls `HttpStream::subscribe_sse` against `/v1/chat/completions` with `"stream": true` in the body, and feeds the resulting `SseStream` through `openai_stream.rs`'s incremental parser (see Streaming Path) to produce a real `Stream<Item = Result<StreamChunk, CompleteError>>` — not a `stream::once` wrapper, same non-negotiable ADR-048 established.
- **`supported_models`** — returns a static, versioned list matching `OpenAiConfig`'s configured model family: `gpt-4o`, `gpt-4o-mini`, `gpt-4-turbo`, `gpt-3.5-turbo`. Metadata only, no network call, matching the trait's non-`async` signature — same pattern as ADR-048's Anthropic table, different data.
- **`model_info`** — looks up context-window/family metadata for a single model id from the same static table; `CompleteError::ModelNotFound` for unrecognized ids (same pattern as ADR-048 and `echo_completer.rs:70`).
- **`list_models`** — returns the full static table as `ModelInfo` entries. OpenAI *does* expose a live `GET /v1/models` endpoint, unlike Anthropic — but this ADR deliberately does not call it: `list_models` is declared non-`async`-network-free by the sibling Anthropic design and by the trait's own default `health_check`/`is_model_available` trail-through, which assume a cheap local lookup. Wiring the live `/v1/models` endpoint would require overriding `health_check` too (its default probes via `list_models`) for no v1 benefit — a symmetrical, documented simplification matching ADR-048's, not a missed capability.
- **`supports`, `is_model_available`, `health_check`** — trait defaults unchanged, same rationale as ADR-048: `list_models` is a pure local lookup with no network cost, so the default health probe is adequate for v1.

### Streaming path — SSE chunk → `StreamChunk`/`StreamDelta`

OpenAI's Chat Completions streaming protocol is **structurally different from Anthropic's**, not just differently-shaped JSON inside the same envelope:

- Anthropic emits **named** SSE events (`SseEvent.event` populated: `content_block_delta`, `message_delta`, `message_stop`, …), each carrying a partial payload scoped to that event type, and requires the parser to track open/closed content-block indices as internal state (ADR-048, Streaming Path).
- OpenAI emits **unnamed** `data: {...}` lines — `SseEvent.event` is `None` on every line — where **each line is a complete, self-describing chunk object** (`{"id", "choices": [{"delta": {"content": ...} | {"tool_calls": [...]}, "finish_reason": ...}]}`), terminated by a **sentinel line** `data: [DONE]` rather than a named terminal event.

Mapping into this crate's existing streaming types (`stream_chunk.rs:6-14`, `stream_delta.rs:6-12`):

| OpenAI SSE line | `SseEvent.event` | `SseEvent.data` | Maps to |
|---|---|---|---|
| Chunk with `choices[0].delta.content` | `None` | `{"choices":[{"delta":{"content":"..."}, "finish_reason":null}], ...}` | `StreamChunk { id, delta: Box::new(StreamDelta { content: Some(text_fragment), tool_calls: None }), finish_reason: None }` |
| Chunk with `choices[0].delta.tool_calls` | `None` | `{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"...","function":{"name":"...","arguments":"..."}}]}}], ...}` | `StreamChunk { delta: Box::new(StreamDelta { content: None, tool_calls: Some(vec![ToolCallDelta{..}]) }), .. }` — OpenAI's `tool_calls[].index` is the accumulation key across fragments (parallel tool calls interleave by index), analogous in *purpose* to Anthropic's content-block index but carried inline per-chunk rather than opened/closed by separate named events |
| Chunk with non-null `choices[0].finish_reason` (`"stop"`, `"length"`, `"tool_calls"`, `"content_filter"`) | `None` | `{"choices":[{"delta":{}, "finish_reason":"stop"}], ...}` | Same `StreamChunk`, `finish_reason: Some(FinishReason::from(finish_reason))` |
| `data: [DONE]` sentinel | `None` | literal string `"[DONE]"` (not JSON) | Terminal chunk, stream ends (matches `StreamChunk::terminal(..)`'s existing convention, same as ADR-048's `message_stop` handling) — the parser must special-case this line as a string comparison *before* attempting JSON deserialization, since `[DONE]` is not valid JSON |
| HTTP-level error response (non-2xx before any `data:` line arrives) | — | — | Same as `complete`'s error mapping: `CompleteError::ProviderError`/`AuthenticationFailed`/`RateLimited` depending on status |

Because there are no named lifecycle events to consume for internal state (no `message_start`/`content_block_start`/`content_block_stop`/`ping` equivalents), `openai_stream.rs`'s parser is **simpler** than Anthropic's: it needs only (a) per-chunk JSON deserialization, (b) tool-call-fragment accumulation keyed by `tool_calls[].index` when a single logical tool call spans multiple chunks, and (c) the `[DONE]`-string special case. It is still a genuine incremental, stateful `futures::Stream` adapter over `SseStream` — not a `stream::once` wrapper — satisfying the same real-streaming bar ADR-048 established.

### Does this duplicate ADR-048's `spi/` code? (required check, not assumed)

Compared line-for-line against ADR-048's design:

- **Request building** (`openai_request.rs` vs `anthropic_request.rs`): no shared logic. Anthropic hoists system messages to a top-level field and uses a `content` blocks array per message; OpenAI keeps a flat `messages` array with a `role` string per entry and no block structure. Nothing to factor out.
- **Response parsing** (`openai_response.rs` vs `anthropic_response.rs`): no shared logic. `choices[0].message`/`finish_reason`/`usage.prompt_tokens` vs Anthropic's `content`/`stop_reason`/`usage.input_tokens` are different field names *and* different nesting depth (OpenAI wraps in a `choices` array to support `n>1` completions; Anthropic does not have an analogous concept). Nothing to factor out.
- **Streaming parser** (`openai_stream.rs` vs `anthropic_stream.rs`): no shared logic, and less than it might first appear — Anthropic's parser is a named-event state machine; OpenAI's is a per-line self-contained-object parser with a string-sentinel terminator. These are different *algorithms*, not the same algorithm over different field names.
- **The one genuinely identical piece of logic**: the HTTP-status-code → `CompleteError` mapping table (401/403 → `AuthenticationFailed`, 429 + `retry-after` header → `RateLimited`, connection failure → `NetworkError`, malformed JSON → `SerializationError`/`ProviderError`) is structurally identical between the two plugins — only the `provider: "anthropic"` vs `provider: "openai"` string tag differs. This is roughly 10-15 lines of `match` arms per plugin.

**Decision: do not factor out a shared crate for that one piece.** Two data points don't justify a shared abstraction (rule of three) for something this small, and ADR-048 itself already precedent-set full independence for exactly this reason — its own Alternatives Considered section rejected building both vendors in one PR specifically because *"the second vendor doesn't test anything about the port that the first one doesn't already prove"*, and the inverse holds for factoring: a shared 15-line helper crate would add a third dependency edge and a versioning surface to coordinate across two otherwise-independent plugin repos, for a savings smaller than the coordination cost. If a third vendor (e.g., a future Gemini or LLMServe plugin per ADR-042's original six-vendor list) also needs identical status-code mapping, *that's* the trigger to extract a small `edge-llm-complete`-adjacent helper (or a `core/` default fn on `CompleteError` itself, e.g. `CompleteError::from_http_status(status, provider, retry_after)`) — not before. `edge-plugin-llm-openai` is built as a fully independent repo, mirroring `edge-plugin-a2a`'s layout, exactly as ADR-048 did for Anthropic.

## What this ADR explicitly does NOT solve

- **Gemini, LLMServe, NVIDIA, OpenRouter, Qwen backends** — out of scope, consistent with ADR-042's original six-vendor list and ADR-048's scope discipline. Not scoped here, not blocked by anything here.
- **OAuth-based vendor auth** — OpenAI's API key model needs no `OAuthTokenSourceFactory`, same as Anthropic; that seam (ADR-033's amendment) is exercised by a future OAuth-based vendor, not this one.
- **Live `GET /v1/models` integration** — OpenAI does expose this endpoint, but `list_models`/`model_info` use a static table here, matching ADR-048's Anthropic design and the trait's non-network-cost assumption baked into the default `health_check`/`is_model_available`. Explicitly deferred, not silently dropped — see `model_info` above.
- **Retry/backoff loop** — `CompleteError::RateLimited { retry_after_ms }` and `NetworkError` are populated correctly, but nothing *consumes* them into an actual retry, same pre-existing gap ADR-045 and ADR-048 both named. Not newly introduced here.
- **Context-window enforcement** — `model_info`'s static table carries a context-window figure, but nothing truncates or rejects an over-limit request before sending it to OpenAI; the vendor's own 400 response is the only backstop today.
- **Cost/usage tracking, prompt-caching-equivalent wiring (OpenAI's `prompt_tokens_details.cached_tokens`), and eval harness integration** — `TokenUsage`'s existing fields are populated from OpenAI's response where present, but no consumer aggregates or bills against them.
- **A shared HTTP-status → `CompleteError` helper crate** — identified above as the one piece of genuinely duplicated logic between this plugin and ADR-048's Anthropic plugin, deliberately not extracted (rule-of-three judgment call, see Decision). Revisit if/when a third vendor plugin is built.
- **Wiring into `edge-llm-runtime` (ADR-045) or `server/scm`'s `provider_svc.rs` (#358)** — this ADR produces the plugin; registering it as a selectable backend (alongside `AnthropicCompleter`) behind either composition root is separate follow-on work, exactly as ADR-048 carved out for its own plugin.
- **Multi-vendor routing/fallback (choosing Anthropic vs OpenAI at runtime)** — that's a `Router`/registry concern for `edge-llm-runtime`, not something either individual `Completer` plugin implements.
- **Reconciling the two legacy `FileCredentialResolver` copies** (`plugins/security/`, non-buildable, and `transport/egress/http/scm/auth/`, buildable but stale v0.3.3) — same pre-existing, unrelated gap ADR-048 flagged; this ADR routes around both the same way, by depending on `edge-security-runtime-credential` directly.
- **Upgrading `transport/`, `plugins/`, and `server/`'s workspace-wide `swe-edge-security` v0.3.3 pin to v0.3.7** — unrelated monorepo-wide version-skew issue, out of scope here, same as ADR-048.

## Consequences

**What this enables**
- The second real, non-echo vendor path for `Completer::complete`/`complete_stream`, closing ADR-048's own explicitly-named follow-up rather than leaving it as a one-line tracking bullet.
- Proof that the `Completer` port is genuinely vendor-agnostic in practice, not just in ADR-048's argument: two vendors with materially different request shapes (system-hoisted vs system-in-array), materially different auth headers (`x-api-key` vs `Authorization: Bearer`), and materially different streaming protocols (named multi-event state machine vs unnamed self-contained lines with a string sentinel) both compile against the same trait, the same `CompleteError`, and the same `StreamChunk`/`StreamDelta` types with zero api/ changes.
- A second worked example for any future third vendor to compare against, sharpening the rule-of-three trigger for eventually extracting the one piece of genuinely shared logic (HTTP-status → `CompleteError` mapping) identified in Decision.
- Real incremental streaming validated against a second, structurally distinct SSE protocol — confirming `swe-edge-egress-http`'s generic `SseEvent` framing is sufficient for both named-event and unnamed-line vendor protocols without any change to the transport crate.

**What this requires**
- New repo scaffold (`sweengineeringlabs/edge-plugin-llm-openai`), mirroring `edge-plugin-llm-anthropic`'s `scm/` layout, which itself mirrors `edge-plugin-a2a`'s reference layout.
- Zero changes to `edge-llm-complete`'s `api/` — confirmed by direct re-read of `completer.rs`, `complete_error.rs`, `stream_chunk.rs`, `stream_delta.rs` for this ADR, not inherited from ADR-048's claim.
- Zero changes to `swe-edge-egress-http` — `HttpEgress`, `HttpStream`, `SseStream` already handle both vendors' framing needs.
- Zero changes to the ADR-015/ADR-048 credential-resolution mechanism — reused as designed, same current `edge-security-runtime-credential` integration point.
- An `OPENAI_API_KEY` (or file-based credential per `CredentialSourceConfig`) available at deployment time — an operational requirement, not a code gap.

## Alternatives Considered

**Factor a shared `edge-llm-complete-vendor-http` (or similar) helper crate for the HTTP-status → `CompleteError` mapping shared between this plugin and ADR-048's Anthropic plugin**
Rejected for now. Only one piece of logic (roughly 10-15 lines of `match` arms) is genuinely identical between the two plugins; everything else — request building, response parsing, streaming state machine — is vendor-specific enough to not be worth abstracting, and duplicating 15 lines twice is cheaper than introducing and versioning a third shared crate across two independent plugin repos. Revisit under a rule-of-three trigger if a third vendor plugin needs the same mapping.

**Call OpenAI's live `GET /v1/models` endpoint for `list_models`/`model_info` instead of a static table**
Rejected for v1, for symmetry with ADR-048's Anthropic design and because the trait's default `health_check`/`is_model_available` assume `list_models` is a cheap local call. Making `list_models` a network call would require also overriding `health_check` (to avoid a live probe on every health check) for no v1 benefit. A static, versioned table is standard practice (matches real-world SDKs) and is a deliberate, documented simplification — not a stub.

**Build this plugin as a sub-module inside `edge-plugin-llm-anthropic` instead of a fully independent repo**
Rejected. Contradicts ADR-048's own precedent (a fully independent repo, `edge-plugin-a2a`-style) and the Decision-section finding above that the two plugins share essentially no code beyond a trivial status-mapping table. A shared repo would couple two vendors' release cadences and CI for no structural benefit.

**Skip real SSE parsing for OpenAI; keep `complete_stream` as a `stream::once`-wrapped call to `complete`, same as the pre-ADR-048 `Completer` implementations**
Rejected, for the same reason ADR-048 rejected it for Anthropic: this is precisely the fakeness the original audit flagged, and OpenAI's SSE format is no harder to parse incrementally than Anthropic's (arguably simpler — see Streaming Path) — declining to use `HttpStream::subscribe_sse` here would be an inconsistency between the two "real" vendor plugins for no reason.

## Tracking

- New repo: `sweengineeringlabs/edge-plugin-llm-openai`
- Depends on (already closed, verified): `edge-domain#77` (`NetworkError`), `edge-domain#78` (`CompletionInput`) — same prerequisites ADR-048 verified, unchanged by this ADR
- Depends on (sibling, independent — not a build dependency, a design precedent): ADR-048 / `edge-plugin-llm-anthropic` — this ADR mirrors its layout and conventions but does not depend on its code
- Follow-up (separate ADR/issue): register `OpenAiCompleter` behind `edge-llm-runtime` (ADR-045) and/or `server/scm`'s `provider_svc.rs` (#358), alongside `AnthropicCompleter`
- Follow-up (separate ADR/issue, blocked on having ≥3 vendor plugins): extract the shared HTTP-status → `CompleteError` mapping identified in Decision, if/when a third vendor plugin is built and the duplication triggers a rule-of-three
- Follow-up (pre-existing, unrelated, inherited from ADR-048): reconcile duplicate `FileCredentialResolver` copies (`plugins/security/` — non-buildable — vs `transport/egress/http/scm/auth/` — stale v0.3.3) against the current `edge-security-runtime-credential`
- Follow-up (pre-existing, unrelated, larger, inherited from ADR-048): upgrade `transport/`, `plugins/`, `server/`, `scm/bootstrap`'s workspace `swe-edge-security` dependency from stale `git tag = "v0.3.3"` to the current post-split `edge-security-*` family (v0.3.7)
- Not blocking this ADR: retry/backoff consumption of `CompleteError::RateLimited`/`NetworkError`, context-window enforcement, cost/usage aggregation, live `/v1/models` integration
