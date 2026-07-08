# ADR-048: Real Vendor `Completer` — Anthropic Messages API as First Backend

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-043 (LLM Complete), [edge ADR-042](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-042-llmprovider-reshape-to-edge-plugin.md) (plugin boundary), ADR-045 (edge-llm-runtime), ADR-015 (Security Layer / credential resolution)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit confirmed: **no real vendor `Completer` implementation exists anywhere in the workspace.** The only implementations of `edge_llm_complete::Completer` are:

- `domain/scm/domain/llm/complete/main/src/core/complete/echo_completer.rs` (`EchoCompleter`) — echoes the last user message back as the completion.
- `domain/scm/domain/llm/complete/main/src/core/complete/noop_completer.rs` (`NoopCompleter`) — returns empty/default responses.
- `domain/scm/domain/llm/provider/main/src/core/provider/provider_completer.rs` (`EchoProviderCompleter`) — adapts `edge_llm_provider`'s `EchoExecutionModel` to the `Completer` port; still echo underneath (`Self::build_model()` constructs an `EchoExecutionModel`, `provider_completer.rs:57-65`).

Every one of these implements `complete_stream` by wrapping a single already-computed value in `stream::once` (`echo_completer.rs:41-50`, `provider_completer.rs:151-167`). There is no incremental token-by-token streaming anywhere — "streaming" today means "a stream with exactly one item."

ADR-042 (2026-06-19) already scoped a real vendor backend as a separate **edge plugin** (`edge-plugin-llm-provider`), not part of any domain crate, reshaping the pre-existing standalone `llmprovider` repo (which historically shipped real Anthropic/OpenAI/LLMServe/NVIDIA/OpenRouter/Qwen backends). ADR-042's dependency-inversion rule is explicit and binding on this ADR too:

```
edge-plugin-llm-provider (plugin)
  └─ depends on → edge-llm-provider  (port contracts)
edge-llm-provider (domain, framework)
  └─ MUST NOT depend on → edge-plugin-llm-provider
```

ADR-042's Implementation Plan step 3 ("Resolve `edge-llm-provider` gaps first") filed four issues against `edge-domain`. Per project memory (`edge-domain#77/#78` closed, 2026-06-19/2026-07-08), all four are now resolved: `Provider::complete`, `NetworkError`, and `CompletionInput` are wired end-to-end (`provider` crate is 183/183). **This ADR's job is narrower than ADR-042's**: ADR-042 reshapes an entire pre-existing multi-vendor repo; this ADR is about standing up the *first* real, minimal, single-vendor `Completer` that proves the pattern — the plugin-boundary rule and dependency direction are inherited from ADR-042 unchanged, not re-derived.

### The exact port to satisfy

`domain/scm/domain/llm/complete/main/src/api/complete/traits/completer.rs:18-83`:

```rust
pub trait Completer: Send + Sync {
    async fn complete(&self, req: CompleteRequest<'_>) -> Result<CompletionResponse, CompleteError>;
    async fn complete_stream(&self, req: CompletionStreamRequest<'_>) -> Result<CompletionStreamResponse, CompleteError>;
    fn supported_models(&self, req: SupportedModelsRequest) -> Result<SupportedModelsResponse, CompleteError>;
    fn supports(&self, req: ModelSupportRequest<'_>) -> Result<ModelSupportResponse, CompleteError> { /* default, calls supported_models */ }
    async fn model_info(&self, req: ModelInfoRequest<'_>) -> Result<ModelInfoResponse, CompleteError>;
    async fn list_models(&self, req: ListModelsRequest) -> Result<ListModelsResponse, CompleteError>;
    async fn is_model_available(&self, req: ModelAvailabilityRequest<'_>) -> Result<ModelAvailabilityResponse, CompleteError> { /* default, calls model_info */ }
    async fn health_check(&self, _req: CompleterHealthCheckRequest) -> Result<CompleterHealthCheckResponse, CompleteError> { /* default, calls list_models */ }
}
```

This is already Request/Response-shaped (arch 0.2.46 mandate) and already has an `Error`-suffixed error type (`CompleteError`, 15 variants, `complete_error.rs:1-81`, including `NetworkError(String)` at line 48 and `StreamError(String)` at line 52 — both needed by a real implementation, already present, nothing to add). `supports`, `is_model_available`, and `health_check` all have default trail-through implementations — a real completer only has to implement `complete`, `complete_stream`, `supported_models`, `model_info`, and `list_models`; the other three come free.

### Credential resolution already has a home — do not reinvent it, and integrate with the *current* `edge-security` family

`server/scm/main/src/saf/provider_svc.rs:1-32` is six `TODO(#358)` comments naming the intended flow:

```
// 2. Instantiating FileCredentialResolver from ADR-015
// 2. Credential resolver — create FileCredentialResolver (from swe-edge-security ADR-015)
```

**Correction (post-review):** an earlier draft of this ADR pointed at `transport/egress/http/scm/auth/main/src/core/credential/file_credential_resolver.rs`. That code is real and tested, but it's built against `swe-edge-security` — a workspace dependency (`edge/scm/Cargo.toml:29`) pinned to `git tag = "v0.3.3"` of `sweengineeringlabs/edge-security.git`. This domain repo (`edge-domain`) already migrated to the *current*, post-split `edge-security-*` family at `tag = "v0.3.7"` (`domain-handler`'s `edge-security-runtime.workspace = true`; see project history, "switch edge-security-* deps from path to git+tag v0.3.7"). `swe-edge-security` v0.3.3 predates that split — no package named `swe-edge-security` or bare `edge-security` exists anywhere in the current `security/` source tree (`security/application/scm/{authn,authz,pipeline}`, `security/runtime/scm/{authz,credential,identity,tls,validation}`); the top-level `edge` repo's `transport/`, `plugins/`, `server/`, and `scm/bootstrap` simply haven't been updated off the stale pre-split tag yet. Depending on it here would pull a real vendor plugin against a known-stale version of the exact family this repo already uses correctly one directory over.

The **current, correct integration point** is `edge-security-runtime-credential` (`security/runtime/scm/credential/`, package `edge-security-runtime-credential`) — a sibling of the `edge-security-runtime` crate `domain-handler`/`edge-llm-provider` already depend on, at the same (current) tag:

```rust
// security/runtime/scm/credential/main/src/api/traits/credential_source_resolver.rs:16-22
pub trait CredentialSourceResolver: Send + Sync {
    fn resolve(&self, req: CredentialSourceResolveRequest) -> Result<CredentialSourceResolveResponse, CredentialError>;
}
```

This is already Request/Response-shaped and `*Error`-suffixed (`CredentialError`) — unlike the legacy `transport/egress/http` copy's positional `&CredentialSourceConfig` argument and `AuthError` — so it also needs no adaptation to satisfy this repo's arch mandate. A real `FileCredentialSourceResolver` `spi/` implementation already exists (`spi/file_credential_source_resolver.rs`, env-var-override → file-path → home-dir-expansion priority, matching ADR-015 R8's intended order) and is obtained via its `saf/` factory: `CredentialSourceResolverFactory::file() -> Box<dyn CredentialSourceResolver>` (`saf/credential/credential_source_resolver_svc_factory.rs:13-15`). ADR-033's own 2026-06-22 amendment (`ADR-033-llm-provider.md:499-519`) specifies the integration shape (`ProviderConfig.credential_source` + an optional `OAuthTokenSourceFactory` for OAuth vendors) — **this ADR reuses that shape**, just resolved through the current crate rather than the stale one. An Anthropic API key is a plain bearer/header credential, not an OAuth flow, so no `OAuthTokenSourceFactory` is needed for v1.

(Note: a second, independently-stale `FileCredentialResolver` copy also exists at `plugins/security/src/core/file_credential_resolver.rs` — its crate, `swe-edge-plugins-security`, depends on `swe-edge-security = { path = "../../../security/scm/security" }`, a path that **does not exist** on disk (`security/scm/` contains only `application/`, `runtime/`, `transport/`, `docs/` — no `security/` subdirectory). That crate cannot build today. Both legacy copies are superseded by `edge-security-runtime-credential` for the purposes of this ADR; reconciling/removing them is a separate, pre-existing monorepo issue this ADR does not fix, flagged for tracking.)

### Real SSE transport already exists too

`transport/egress/http/scm/transport/main/src/api/traits/http/http_stream.rs:20-31` already defines:

```rust
pub trait HttpStream: Send + Sync {
    fn subscribe_sse(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<SseStream>>;
    fn connect_websocket(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<WsChannel>>;
}
```

where `SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpEgressError>> + Send>>` (`sse_stream.rs:14`) and `SseEvent { event: Option<String>, data: String, id: Option<String> }` (`sse_event.rs:5-12`) already parses `text/event-stream` framing (`event:`/`data:`/`id:` fields) generically. This means the hard part — chunked-transfer SSE line framing — is not a gap this ADR needs to fill. What's missing is purely vendor-specific: parsing Anthropic's JSON payload *inside* `SseEvent.data` and mapping it to `StreamChunk`/`StreamDelta`.

## Decision

Build a new standalone plugin repo, **`edge-plugin-llm-anthropic`**, that implements `edge_llm_complete::Completer` against Anthropic's Messages API (`POST /v1/messages`), following ADR-042's plugin-boundary rule exactly: it depends on `edge-llm-complete` (port contracts) and `swe-edge-egress-http` (HTTP/SSE transport) as ordinary library dependencies; neither of those crates gains a dependency back on this plugin.

### Why Anthropic first, and why not both vendors at once

Anthropic's Messages API is recommended as the first vendor:

1. **Platform alignment** — this entire monorepo (`edge`) is built around Claude/Claude Code as the primary consumer and reference model; proving the real end-to-end path against the vendor the platform already assumes is the highest-leverage first proof.
2. **`Completer` is already vendor-agnostic** — nothing in the trait (`completer.rs`), the request/response types (`CompletionRequest`, `CompletionResponse`, `Message`, `MessageContent`), or the streaming types (`StreamChunk`, `StreamDelta`, `ToolCallDelta`) is Anthropic-specific. An OpenAI-compatible (`/v1/chat/completions`) implementation is an equally cheap, structurally identical follow-on — same trait, same SSE transport trait, different JSON shape and a different vendor auth header. Building both in v1 would double the surface (two vendor SPI modules, two credential configs, two sets of vendor-response fixtures) for no validation benefit: the second vendor doesn't test anything about the port that the first one doesn't already prove.
3. **Scope discipline** — the historical `llmprovider` repo ADR-042 reshapes already had six vendors (Anthropic, OpenAI, LLMServe, NVIDIA, OpenRouter, Qwen) and ADR-042 explicitly notes `complete_stream` was only implemented for two of them (Anthropic, NVIDIA) even there. Chasing multi-vendor parity in a *first* real implementation repeats that scope creep. One vendor, fully real (blocking + streaming + model listing + health check), is a stronger deliverable than N vendors partially real.

Gemini is explicitly out of scope here too, consistent with ADR-042 (`spi/gemini/` stub only, tracked separately).

### Shape / workspace layout

```
edge-plugin-llm-anthropic/                      (new standalone repo, mirrors edge-plugin-a2a's scm/ layout)
└── scm/
    └── main/src/
        ├── api/
        │   └── types/anthropic_config.rs        (AnthropicConfig: model defaults, base_url override, credential_source)
        ├── core/
        │   └── anthropic_completer.rs            (AnthropicCompleter: impl Completer)
        ├── spi/
        │   ├── anthropic_request.rs              (CompletionRequest → Anthropic Messages JSON body)
        │   ├── anthropic_response.rs              (Anthropic JSON response → CompletionResponse)
        │   └── anthropic_stream.rs                (SseEvent → StreamChunk/StreamDelta incremental parser)
        └── saf/
            └── anthropic_completer_svc.rs         (anthropic_completer(config, http_stream, http_egress) -> impl Completer)
```

Depends on: `edge-llm-complete` (`Completer`, `CompleteError`, `CompletionRequest`/`Response`, `StreamChunk`/`StreamDelta`, `ToolCallDelta`), `swe-edge-egress-http` (`HttpEgress`, `HttpStream`, `SseStream`, `SseEvent`), `edge-security-runtime-credential` (`CredentialSourceResolver`, `CredentialSourceResolveRequest`/`Response`, `CredentialSourceResolverFactory::file()`) — at the same current tag `edge-llm-provider` already resolves `edge-security-runtime` against, not the stale `swe-edge-security` v0.3.3.

### `AnthropicCompleter` — mapping each trait method

- **`complete`** — builds an Anthropic Messages request body from `CompletionRequest.messages` (role/content mapping: `Role::User`→`"user"`, `Role::Assistant`→`"assistant"`, system messages hoisted to the top-level `system` field per Anthropic's API shape, not left in the `messages` array), calls `HttpEgress::send` with the resolved API key in the `x-api-key` header, and maps the JSON response body into `CompletionResponse` (`content`, `finish_reason`, `usage` from Anthropic's `stop_reason`/`usage.input_tokens`/`usage.output_tokens`). Non-2xx / malformed-JSON responses map to `CompleteError::ProviderError { provider: "anthropic", message }`; connection failures map to `CompleteError::NetworkError`; a 401/403 maps to `CompleteError::AuthenticationFailed`; 429 maps to `CompleteError::RateLimited { retry_after_ms }` read from Anthropic's `retry-after` header when present.
  **Correction (post-review):** an earlier draft of this mapping omitted `Role::Tool` (`complete/main/src/api/complete/types/role.rs:15`) entirely — a real gap, not a cosmetic one, since `Role::Tool` messages are genuinely produced by the tool-call loop (`complete/main/src/core/complete/message.rs:39`, exercised by `complete/tests/tool_call_loop_e2e_test.rs:183`) and Anthropic's API has no flat `"tool"` role at all: a tool result must be nested as a `tool_result` content block inside a `user`-role message, keyed by the tool-call id it answers. `AnthropicCompleter`'s mapper folds each `Role::Tool` message into the *preceding* `user` turn's content-block array as a `{"type": "tool_result", "tool_use_id": ..., "content": ...}` block rather than emitting it as its own top-level message — the one role that doesn't map 1:1 to a top-level Anthropic message.
- **`complete_stream`** — calls `HttpStream::subscribe_sse` against `/v1/messages` with `"stream": true` in the body, and feeds the resulting `SseStream` through `anthropic_stream.rs`'s incremental parser (see Streaming Path below) to produce a real `Stream<Item = Result<StreamChunk, CompleteError>>` — not a `stream::once` wrapper.
- **`supported_models`** — returns a static, versioned list (`claude-opus-4`, `claude-sonnet-4-5`, …) matching `AnthropicConfig`'s configured model family; this is metadata, not a network call, matching the trait's non-`async` signature.
- **`model_info`** — looks up context-window/family metadata for a single model id from the same static table; `CompleteError::ModelNotFound` for unrecognized ids (mirrors `echo_completer.rs:70`'s existing pattern for this exact error path).
- **`list_models`** — returns the full static table as `ModelInfo` entries. (Anthropic's Messages API has no live model-listing endpoint at present; a static, versioned table is standard practice and is what real-world SDKs do too — this is a deliberate, documented simplification, not a stub.)
- **`supports`, `is_model_available`, `health_check`** — use the trait's default implementations unchanged (`completer.rs:39-46`, `61-69`, `76-82`); `health_check`'s default (probe via `list_models`) is adequate since `list_models` here is a pure local lookup with no network cost, so no override is needed for v1.

### Streaming path — SSE chunk → `StreamChunk`/`StreamDelta`

Anthropic's Messages streaming protocol emits a sequence of named SSE events (`message_start`, `content_block_start`, `content_block_delta`, `content_block_stop`, `message_delta`, `message_stop`), each with a JSON payload in `SseEvent.data`. The mapping into this crate's existing streaming types (`stream_chunk.rs:6-14`, `stream_delta.rs:6-12`):

| Anthropic SSE event | `SseEvent.event` | Maps to |
|---|---|---|
| `content_block_delta` (text delta) | `"content_block_delta"` | `StreamChunk { id, delta: Box::new(StreamDelta { content: Some(text_fragment), tool_calls: None }), finish_reason: None }` |
| `content_block_delta` (tool-use input-json delta) | `"content_block_delta"` | `StreamChunk { delta: Box::new(StreamDelta { content: None, tool_calls: Some(vec![ToolCallDelta{..}]) }), .. }` |
| `message_delta` (carries `stop_reason`) | `"message_delta"` | Same `StreamChunk`, `finish_reason: Some(FinishReason::from(stop_reason))` |
| `message_stop` | `"message_stop"` | Terminal chunk, stream ends (matches `StreamChunk::terminal(..)`'s existing convention already used by `EchoCompleter`/`EchoProviderCompleter`) |
| `ping`, `message_start`, `content_block_start`, `content_block_stop` | — | Consumed internally by the parser for state tracking (e.g. tracking which content-block index is open); emit no `StreamChunk` |
| `error` | `"error"` | Parser maps to `Err(CompleteError::StreamError(message))`, terminating the mapped stream |

This is a genuine incremental parser — a stateful `futures::Stream` adapter over the `SseStream`, not a single collected-then-replayed chunk. It is the first place in the workspace `StreamChunk`/`StreamDelta` are populated from a real multi-event source rather than constructed once and wrapped in `stream::once` (contrast `echo_completer.rs:46`, `provider_completer.rs:157-161`).

## What this ADR explicitly does NOT solve

- **OpenAI (or any other vendor) backend** — deliberately deferred as an independent, equally-cheap follow-on (see Decision, "Why Anthropic first"). Not scoped here, not blocked by anything here.
- **Gemini** — out of scope, consistent with ADR-042.
- **OAuth-based vendor auth** — Anthropic's API key model needs no `OAuthTokenSourceFactory`; that seam (already specified in ADR-033's amendment) is exercised by a future OAuth-based vendor, not this one.
- **Retry/backoff loop** — `CompleteError::RateLimited { retry_after_ms }` and `NetworkError` are populated correctly, but nothing *consumes* them into an actual retry (same gap ADR-045 named for `ExecutionError::is_retryable/retry_after`). Left inert by design, tracked as existing follow-up work, not newly introduced here.
- **Context-window enforcement** — `model_info`'s static table carries a context-window figure, but nothing truncates or rejects an over-limit request before sending it to Anthropic; the vendor's own 400 response is the only backstop today.
- **Cost/usage tracking, prompt caching (`cache_read_input_tokens`/`cache_creation_input_tokens` wiring), and eval harness integration** — `TokenUsage`'s existing fields for these are populated from Anthropic's response where present, but no consumer aggregates or bills against them.
- **Wiring into `edge-llm-runtime` (ADR-045) or `server/scm`'s `provider_svc.rs` (#358)** — this ADR produces the plugin; registering it as the live backend behind either composition root is separate follow-on work, exactly as ADR-045 already carved out ("Real vendor `Completer` — separate ADR/issue, explicitly out of scope here").
- **Reconciling the two legacy `FileCredentialResolver` copies** (`plugins/security/` — which cannot currently build, its path dependency points at a nonexistent `security/scm/security/` — and `transport/egress/http/scm/auth/`, which builds but against the stale `swe-edge-security` v0.3.3) — flagged under Context; this ADR routes around both by depending on `edge-security-runtime-credential` directly, but does not delete or fix either legacy copy.
- **Upgrading `transport/`, `plugins/`, and `server/`'s workspace-wide `swe-edge-security` v0.3.3 pin to the current `edge-security-*` family (v0.3.7)** — a monorepo-wide version-skew issue this ADR's plugin simply avoids by not depending on the stale crate at all. Fixing the pin itself is separate, unrelated follow-on work, out of scope here.

## Consequences

**What this enables**
- The first real, non-echo path from `Completer::complete`/`complete_stream` to an actual LLM vendor, closing the single most-cited gap across ADR-045 and ADR-046 ("no real vendor `Completer` (still echo/noop only)").
- A concrete, working reference for the second vendor (OpenAI-compatible) to copy structurally — same `spi/` shape (`*_request.rs`/`*_response.rs`/`*_stream.rs`), same `saf/` factory pattern, different JSON mapping.
- Real incremental streaming validated end-to-end for the first time — `StreamChunk`/`StreamDelta` proven against a genuine multi-event source, not just constructed once in a test.
- Credential handling proven against the *current* `edge-security-runtime-credential` mechanism exactly as ADR-033's amendment already specified, closing out the design half of `edge-domain#358`'s TODOs (the assembler-wiring half remains ADR-045's follow-up, not this ADR's) — and doing so without adding a dependency on the stale, pre-split `swe-edge-security` v0.3.3 that `transport/`, `plugins/`, and `server/` are still pinned to.

**What this requires**
- New repo scaffold (`sweengineeringlabs/edge-plugin-llm-anthropic`), mirroring `edge-plugin-a2a`'s `scm/` layout (the reference plugin at 174/174 per ADR-042's references).
- Zero changes to `edge-llm-complete`'s `api/` — `Completer`, `CompleteError`, `StreamChunk`/`StreamDelta` are all already sufficient as-is (see Context).
- Zero changes to `swe-edge-egress-http` — `HttpEgress`, `HttpStream`, `SseStream` already exist and are already sufficient.
- Zero changes to the ADR-015 credential-resolution mechanism — reused as designed.
- An `ANTHROPIC_API_KEY` (or file-based credential per `CredentialSourceConfig`) available at deployment time — an operational requirement, not a code gap.

## Alternatives Considered

**Reshape the historical `llmprovider` repo's existing Anthropic `spi/` code directly, per ADR-042, instead of writing a new minimal plugin**
Rejected for v1, not forever. ADR-042 is a large reshape of a six-vendor repo with its own competing trait hierarchy (`Provider`/`LlmComplete`/`Router`/`Registry`) that must first be stripped out; ADR-042 itself sequences that as a multi-step migration (steps 1–12) gated on four `edge-domain` issues that only just closed. Standing up one new, small, purpose-built plugin against the now-stable `Completer` port proves the pattern faster and de-risks ADR-042's larger reshape — once this plugin is real and working, ADR-042's Anthropic `spi/` code becomes one of (at least) two independently-validated implementations to reconcile, rather than the only one.

**Implement the vendor call directly inside `edge-llm-complete`'s `core/`**
Rejected. Violates ADR-042's dependency-inversion rule verbatim: "the framework's public contract is byte-for-byte identical whether or not any plugin exists." An HTTP client, an API key, and vendor-specific JSON parsing are exactly the kind of transport/vendor detail ADR-042 already ruled must live in a plugin, not a domain crate.

**Build both Anthropic and OpenAI in this same ADR/PR**
Rejected for v1. See Decision — doubles surface area (two SPI modules, two credential configs, two vendor fixture sets) without validating anything about the port a single vendor doesn't already prove. Filed as an explicit, cheap follow-on instead.

**Skip real SSE parsing; keep `complete_stream` as a `stream::once`-wrapped call to the blocking `complete` method, same as every existing `Completer`**
Rejected. This is precisely the fakeness ADR-045 flagged ("every `Completer` impl only emits a single `stream::once` chunk") and the audit that produced this ADR was scoped to fix. `swe-edge-egress-http`'s `HttpStream::subscribe_sse`/`SseStream` already exist for exactly this purpose (see Context) — not using them would be declining free, already-built infrastructure.

## Tracking

- New repo: `sweengineeringlabs/edge-plugin-llm-anthropic`
- Depends on (already closed, verified): `edge-domain#77` (`NetworkError`), `edge-domain#78` (`CompletionInput`) — both confirmed resolved per 2026-07-08 provider audit
- Follow-up (independent, not blocking): `edge-plugin-llm-openai` — same shape, different vendor JSON mapping and auth header
- Follow-up (separate ADR/issue): register `AnthropicCompleter` behind `edge-llm-runtime` (ADR-045) and/or `server/scm`'s `provider_svc.rs` (#358)
- Follow-up (pre-existing, unrelated): reconcile duplicate `FileCredentialResolver` copies (`plugins/security/` — currently non-buildable, broken path dep — vs. `transport/egress/http/scm/auth/` — buildable but stale v0.3.3) against the current `edge-security-runtime-credential`
- Follow-up (pre-existing, unrelated, larger): upgrade `transport/`, `plugins/`, `server/`, `scm/bootstrap`'s workspace `swe-edge-security` dependency from stale `git tag = "v0.3.3"` to the current post-split `edge-security-*` family (v0.3.7) already used throughout `domain/`
- **Revisit once `transport/` is cleaned to depend on the current `edge-security` tag:** re-evaluate whether `edge-plugin-llm-anthropic` should switch its credential resolution from `edge-security-runtime-credential` (direct, this ADR's choice) to `swe-edge-egress-auth` (transport's own HTTP-auth crate), so credential resolution for HTTP-egress plugins is centralized behind one transport-layer seam instead of each plugin reaching into `edge-security-runtime-credential` independently. **Contingent on**: confirming that cleanup also reshapes `swe-edge-egress-auth`'s `CredentialSourceResolver::resolve(&self, config: &CredentialSourceConfig) -> Result<CredentialSource, AuthError>` to the mandatory Request/Response port shape (a version-pin bump alone does not fix that; it's a separate, orthogonal defect from the staleness this ADR routed around). If the cleanup is version-only, this plugin should keep depending on `edge-security-runtime-credential` directly rather than inherit a still-non-compliant shape.
- Not blocking this ADR: retry/backoff consumption of `CompleteError::RateLimited`/`NetworkError`, context-window enforcement, cost/usage aggregation
