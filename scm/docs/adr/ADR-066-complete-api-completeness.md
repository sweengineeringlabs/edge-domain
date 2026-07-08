# ADR-066: `edge-llm-complete` API Completeness — Async `ToolOps`, Request Timeout, `ModelCapability` Enum, Cached-System Builder

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-043 (LLM Complete Domain Primitive), ADR-056 (Multimodal Vision Input — touches `ModelInfo.supports_vision`, checked for conflict below), ADR-050 (Context-Window Guard — reads a *different* `ModelInfo`, checked for interaction below), ADR-068 (`CacheableMessage` Consumer — the real consumer of `cache_control`; complementary to this ADR's producer-side builder)
**GitHub Issues:** `sweengineeringlabs/edge-domain#81`, `#82`, `#83`, `#84`

---

## Context

The 2026-07-08 LLM-landscape audit that produced ADR-045/046/048/050/056/068 also surfaced four small, already-filed API-completeness gaps against `edge-llm-complete`, all four tagged in their own issues as `ADR-043 §Known Limitations L1`–`L4`. Each is real but too small to justify its own ADR on its own, and all four touch the same small set of files (`api/complete/traits/tool_ops.rs`, `api/complete/types/completion_request.rs`, `api/complete/types/model_info.rs`), so this ADR bundles them, verifies each issue's proposed shape against the code as it exists today, and settles the design details the issues left open (exact types, exact blast radius, exact interaction with ADR-050/056/068).

Each of the four had drifted from the code in at least one respect by the time this ADR was written — most importantly, `ToolOps` and `CacheableMessage` are *already* Request/Response-shaped (arch 0.2.46's mandate), which the issues' illustrative code snippets (written before that conversion landed) do not reflect. This ADR designs against the real, current signatures, not the issues' snippets verbatim.

## Decision

### 1. (#81) `ToolOps::execute()` becomes `async`

**Verified current state** — `domain/scm/domain/llm/complete/main/src/api/complete/traits/tool_ops.rs:10-31`:

```rust
pub trait ToolOps: Send + Sync {
    fn execute(&self, req: ToolExecutionRequest<'_>) -> Result<ToolExecutionResponse, CompleteError>;
    fn available_tools(&self, req: AvailableToolsRequest) -> Result<AvailableToolsResponse, CompleteError>;
    fn tool_choice(&self, req: ToolChoicePreferenceRequest) -> Result<ToolChoicePreferenceResponse, CompleteError>;
    fn merge_delta(&self, req: DeltaMergeRequest<'_>) -> Result<(), CompleteError>;
}
```

`execute` is still `fn`, still synchronous, exactly as issue #81 describes — but unlike the issue's illustrative snippet (which showed `&ToolCall`/`Result<String, CompleteError>`), the real signature is already Request/Response-shaped (`ToolExecutionRequest<'_>` / `ToolExecutionResponse`) and already `*Error`-suffixed (`CompleteError`). Nothing about the port shape needs redesigning — this is purely adding `async` to one method.

**Change:**

```rust
#[async_trait]
pub trait ToolOps: Send + Sync {
    async fn execute(&self, req: ToolExecutionRequest<'_>) -> Result<ToolExecutionResponse, CompleteError>;
    fn available_tools(&self, req: AvailableToolsRequest) -> Result<AvailableToolsResponse, CompleteError>;
    fn tool_choice(&self, req: ToolChoicePreferenceRequest) -> Result<ToolChoicePreferenceResponse, CompleteError>;
    fn merge_delta(&self, req: DeltaMergeRequest<'_>) -> Result<(), CompleteError>;
}
```

`available_tools`, `tool_choice`, and `merge_delta` stay synchronous — none of them do I/O (`merge_delta` mutates an in-memory delta; the other two return static/derived metadata). Only `execute` is the method real tool backends (web search, code execution, external API calls) need to await inside. This mirrors the existing pattern on `Completer` itself (`api/complete/traits/completer.rs:3,17`: `use async_trait::async_trait; #[async_trait] pub trait Completer`), which already mixes async and sync methods on one trait (`supported_models`/`supports` stay sync at lines 30-33/36-39 while `complete`/`model_info`/`list_models` are async) — the same mixed shape, not a new pattern.

**Every implementer, found by `grep -r "impl ToolOps" domain/scm/domain/llm`:**

| Site | Change needed |
|---|---|
| `complete/main/src/core/complete/noop_completer.rs:102-140` (`impl ToolOps for NoopCompleter`) | Add `#[async_trait]`, make `execute` (lines 103-110) `async fn` |
| `complete/tests/tool_ops_e2e_test.rs:13-55` (`impl ToolOps for EchoToolOps`) | Same — `execute` at lines 14-27; all six scenario tests (`test_execute_*`) become `#[tokio::test]` and gain `.await` |
| `complete/tests/tool_call_loop_e2e_test.rs:87-111` (`impl ToolOps for FailingToolOps`) | Same — `execute` at lines 88-93 |
| `complete/tests/tool_call_loop_e2e_test.rs:114-140` (`impl ToolOps for EchoToolOps`) | Same — `execute` at lines 115-122 |

**The one real (non-test) call site**, `core/complete/tool_call_step.rs:36-38`:

```rust
let response = self.tool_ops.execute(ToolExecutionRequest { call: &self.call })?;
```

is inside `DefaultToolCallStep::execute` (`tool_call_step.rs:32-45`), which is *already* `#[async_trait] impl Step` with `async fn execute` (lines 27-32). So the production call site needs only `.await?` appended — it is already running inside an async context; no caller up the chain (`BoundedToolCallLoop::run`, `bounded_tool_call_loop.rs:24-99`, already `async fn`, already awaits the sibling `Completer::complete` call at line 44) needs to change shape, only to `.await` one more thing. This is the correctness win the issue names: today, an implementer needing async work inside `execute` has no legal way to get it without `block_on()`; after this change the one production caller already has an `.await` point ready to receive it.

### 2. (#82) `CompletionRequest.timeout_ms`

**Verified current state** — `api/complete/types/completion_request.rs:6-24`: a plain `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]` struct, 8 fields, no field named `timeout_ms`. `CompleteError::Timeout(u64)` already exists (`api/complete/errors/complete_error.rs:56`) — the error variant the field is meant to be paired with is already there.

**Change** — add one field:

```rust
pub struct CompletionRequest {
    // ...existing 8 fields...
    /// Per-request timeout in milliseconds; `None` defers to the completer's own default.
    pub timeout_ms: Option<u64>,
}
```

`u64` (not `u32`, matching issue #82's own struct sketch and `CompleteError::Timeout(u64)`'s existing payload type — `u32` would require a lossy cast at the point the error is raised).

**Construction sites, verified by grepping every `CompletionRequest {` literal in `domain/scm/domain/llm`:** there is exactly **one** exhaustive struct literal in the entire tree that does not use `..Default::default()` or a `..spread`: `provider/main/src/core/provider/completion/completion_input.rs:58-72`, inside `CompletionInput::into_completion_request`, which lists all 8 current fields by name (`model`, `messages`, `temperature`, `max_tokens`, `top_p`, `stop`, `tools`, `tool_choice`) with no spread. **This is a real, cross-crate compile break in `edge-llm-provider`**, not just a same-crate test fixup as issue #82's "Impact" section implies (it only names this crate's own tests) — the fix is a one-line addition, `tool_choice: None, timeout_ms: None,`, but it is a required, not optional, follow-up in a different crate. Every other construction site in the tree (`bounded_tool_call_loop.rs:37-40` via `..req.request.clone()`; `CompletionRequest::new(...)` at `core/complete/completion/completion_request.rs:7-13`, itself using `..Default::default()`; and every test file found) already spreads or goes through `::new()`, so they compile unchanged. Issue #82 also names `CompleteFactory::request()` as a call site to check — no `CompleteFactory` type exists anywhere in this crate (confirmed by grep); that reference in the issue is stale and this ADR does not act on it.

No builder method is added for `timeout_ms` itself beyond the existing `..Default::default()` / struct-update pattern already used throughout the crate — a dedicated `.with_timeout_ms(ms)` was considered but rejected as unnecessary ceremony for a single `Option<u64>` field callers can already set via struct-update syntax (see Alternatives Considered).

### 3. (#83) `ModelInfo` capability booleans → `ModelCapability` enum set

**Verified current state** — `api/complete/types/model_info.rs:4-20`:

```rust
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_window: u32,
    pub supports_vision: bool,
    pub supports_function_calling: bool,
    pub supports_streaming: bool,
}
```

**This is `edge_llm_complete::ModelInfo` — confirmed distinct from `edge_llm_provider::ModelInfo`.** ADR-050 already documented (and left unresolved, by design) that **two independent `ModelInfo` types** exist in the workspace: `edge_llm_provider::api::provider::types::ModelInfo` (`provider/main/src/api/provider/types/model_info.rs`, fields `id, name, family, context_window, supports_vision, supports_functions, supports_streaming, training_cutoff` — note the *different* field name `supports_functions`, plus `family`/`training_cutoff` this crate's type doesn't have) is the one `StdProvider` actually holds (`provider/main/src/api/provider/types/std_provider.rs:17`, `pub(crate) model: Option<ModelInfo>`). This ADR touches **only** `edge_llm_complete::ModelInfo`; `edge_llm_provider::ModelInfo` is untouched, unrelated, and out of scope, exactly as ADR-050 scoped it ("Does not fix the two-`ModelInfo`-types duplication... a separate, narrower cleanup, not bundled here").

**Blast radius inside `edge-llm-complete`, verified by grepping `supports_vision|supports_function_calling|supports_streaming` restricted to this crate** — exactly four hits, all internal:

- `api/complete/types/model_info.rs:15,17,19` — the fields themselves (this change)
- `core/complete/model_info.rs:5-28` — `ModelInfo::new(...)` constructor, sets all three `false` (lines 18-20); also read directly by `api/complete/traits/model_ops.rs:17-28`'s default `ModelOps::create_model_info` helper, which delegates to `ModelInfo::new` and needs no signature change, only to compile against the new field shape
- `complete/tests/model_info_e2e_test.rs:14-17` — `test_capabilities_default_to_false` reads all three as bare fields (`!m.supports_vision && !m.supports_function_calling && !m.supports_streaming`)
- `complete/tests/model_ops_e2e_test.rs:64` — same bare-field read pattern

**No consumer outside `edge-llm-complete`'s own `main/src`/`tests/` reads these three fields** — the wider grep across all of `domain/scm/domain/llm` for the same three names surfaces only `edge_llm_provider`'s *own*, textually-similar-but-unrelated fields on its own `ModelInfo`, plus ADR markdown files that discuss the gap (ADR-048, ADR-050, ADR-056, ADR-052, ADR-054, ADR-069) without any of them constructing or reading `edge_llm_complete::ModelInfo`'s booleans in code. **This is the smallest-blast-radius change of the four** despite issue #83 flagging it as "the most invasive" — it is invasive to its *own* two call sites (a `false, false, false` constructor and two field-access tests), not to the rest of the workspace.

**Change:**

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ModelCapability {
    Vision,
    FunctionCalling,
    Streaming,
}

pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_window: u32,
    pub capabilities: HashSet<ModelCapability>,
}

impl ModelInfo {
    pub fn supports(&self, cap: ModelCapability) -> bool {
        self.capabilities.contains(&cap)
    }
    pub fn supports_vision(&self) -> bool { self.supports(ModelCapability::Vision) }
    pub fn supports_function_calling(&self) -> bool { self.supports(ModelCapability::FunctionCalling) }
    pub fn supports_streaming(&self) -> bool { self.supports(ModelCapability::Streaming) }
}
```

`HashSet<ModelCapability>` (not `Vec`) so `supports()` is O(1) and duplicate capabilities can't be inserted twice — a `Vec` was issue #83's own sketch but a set is the tighter fit given `ModelCapability` already derives `Eq + Hash` in that same sketch. Only the three capabilities the crate has an actual bool for today (`Vision`, `FunctionCalling`, `Streaming`) are added now; issue #83's suggested future variants (`StructuredOutput`, `AudioInput`, `AudioOutput`, `PromptCaching`) are **not** added speculatively in this ADR — an enum variant with zero producers or consumers is exactly the "declare and abandon" anti-pattern ADR-056 named for `supports_vision` itself. Adding a capability is a cheap, additive, non-breaking follow-up (new enum variant, no struct shape change) whenever a real consumer needs one.

`ModelInfo::new(...)` (`core/complete/model_info.rs:7-22`) changes from setting three `false` bools to `capabilities: HashSet::new()` — an empty set is the direct semantic equivalent of "all capabilities false," so `test_new_sets_id_and_defaults_capabilities_false` (`model_info.rs:36-40`) and `test_capabilities_default_to_false` (`model_info_e2e_test.rs:14-17`) both still hold, just rewritten against `.supports_vision()` (method call) instead of `.supports_vision` (field).

**Interaction with ADR-056 — checked, no conflict, by design:** ADR-056's Context section (`ADR-056-multimodal-vision-input.md:13`) cites this exact file (`complete/main/src/api/complete/types/model_info.rs:15`) as its motivating "declare and abandon" example, but ADR-056's actual runtime gate (`VisionCapabilityRequest.model_supports_vision`, wired at `std_provider.rs:118-125`) is populated from `Provider::model_info()` — which returns `edge_llm_provider::ModelInfo`, the *other* type, per ADR-050's finding above. So ADR-056's gate is not, in fact, reading the field this ADR is changing. More directly, ADR-056 says so itself, explicitly punting on this exact question (`ADR-056-multimodal-vision-input.md:153`): *"Does not touch the `supports_vision`-is-a-plain-`bool` shape question raised in GitHub issue #83... This ADR reads the existing bool as-is; the type-shape question is orthogonal and tracked separately."* This ADR is that tracked-separately follow-up. Keeping `.supports_vision()` as a same-named accessor method (rather than dropping it) means any future code written against ADR-056's pattern, or against `edge_llm_complete::ModelInfo` directly, reads almost identically to before (`.supports_vision` → `.supports_vision()`), minimizing churn if the two `ModelInfo` types are ever unified (a cleanup both ADR-050 and this ADR leave for later).

**Interaction with ADR-050 — checked, no interaction.** ADR-050's context-window guard reads `edge_llm_provider::ModelInfo.context_window` exclusively (`ADR-050-context-window-guard.md:120,131`) and never touches capability booleans on either `ModelInfo` type. No change here affects it.

### 4. (#84) `CompletionRequest::with_cached_system()` builder

**Verified current state** — issue #84's sketch assumes `Message::system(content).mark_ephemeral()` returns `Self` directly and chains straight into `Vec::insert`. The real signatures are Request/Response-shaped and fallible:

- `Message::system(content: impl Into<String>) -> Self` — `core/complete/message.rs:28-34`, infallible, as sketched.
- `CacheableMessage::mark_ephemeral(self, MarkEphemeralRequest) -> Result<CacheControlResponse<Self>, CompleteError>` — `api/complete/traits/cacheable_message.rs:16-24`, a *Request-taking, Result-returning* method (`MarkEphemeralRequest` is a unit marker struct at `api/complete/types/mark_ephemeral_request.rs:3`), not the bare `self -> Self` chain the issue sketches. The concrete `impl CacheableMessage for Message` (`core/complete/message.rs:47-54`) never actually fails — `with_cache_control` unconditionally returns `Ok(...)` — but the trait's public signature is `Result`-typed, so any caller of it must handle that, per the trait's own contract.

**As flagged in the task brief, this is the first real consumer of `Message.cache_control`/`CacheableMessage` inside `edge-llm-complete` itself.** A repo-wide grep of `cache_control` (also performed independently by ADR-068, which cites the same 71 hits) confirms every existing use is either the type/trait/constructor itself or a test asserting the field got set on a bare Rust struct — nothing today calls `mark_ephemeral` from anywhere other than a test. `with_cached_system` changes that: it is production code (not a test) that calls `mark_ephemeral` to build a request users actually send.

**Change:**

```rust
impl CompletionRequest {
    /// Prepend a system message marked for Anthropic prompt caching via [`CacheableMessage::mark_ephemeral`].
    pub fn with_cached_system(mut self, content: impl Into<String>) -> Result<Self, CompleteError> {
        let sys = Message::system(content)
            .mark_ephemeral(MarkEphemeralRequest)?
            .message;
        self.messages.insert(0, sys);
        Ok(self)
    }

    /// Prepend a plain (non-cached) system message.
    pub fn with_system(mut self, content: impl Into<String>) -> Self {
        self.messages.insert(0, Message::system(content));
        self
    }
}
```

**Deliberate divergence from issue #84's sketch:** `with_cached_system` returns `Result<Self, CompleteError>`, not bare `Self`. The issue's snippet returns `Self` directly, which would require either `.expect(...)`/`.unwrap()` inside the builder (a panic risk in library code, forbidden by this project's error-handling rule) or silently discarding a `CompleteError` the trait's own signature says can occur. Since `mark_ephemeral`'s real signature is fallible, `with_cached_system` stays honest about that rather than papering over it — `with_system` (the non-cached sibling) has no such constraint and stays infallible, matching the issue's original shape for that half.

This ADR only designs the *builder* (setting `cache_control` on a system message before the request leaves `edge-llm-complete`). **ADR-068 designs the actual consumer** — `AnthropicCompleter`'s request mapping (`spi/anthropic_request.rs`, per ADR-048's shape) reading `Message.cache_control` and emitting Anthropic's `"cache_control": {"type": "ephemeral"}` JSON on the outgoing content block. This ADR does not re-derive that mapping; the two are complementary (producer-side convenience here, consumer-side wire mapping there), as ADR-068 itself states in its own "Relates to" line.

## What this ADR explicitly does NOT solve

- **Does not implement the async tool-execution runtime** — #81 makes `ToolOps::execute` callable with `.await`; it does not add retry, timeout, or cancellation around that await. `CompleteError::Timeout` exists but nothing wraps a slow tool call in a deadline (same class of gap ADR-045 named for `edge-dispatcher`'s `TimeoutHandler` at the HTTP layer, unrelated here).
- **Does not consume `CompletionRequest.timeout_ms`** — #82 adds the field so a value can be carried on the request; no `Completer` implementation (`EchoCompleter`, `NoopCompleter`, or any future vendor backend from ADR-048) reads it yet and enforces a deadline. That wiring is vendor-plugin work, tracked under ADR-048, not this ADR.
- **Does not unify `edge_llm_complete::ModelInfo` and `edge_llm_provider::ModelInfo`** — #83 only reshapes this crate's own type. The two-type duplication ADR-050 already flagged remains, unresolved, exactly as ADR-050 scoped it.
- **Does not add capability variants beyond the three that already have a bool today** (`Vision`, `FunctionCalling`, `Streaming`). `StructuredOutput`/`AudioInput`/`AudioOutput`/`PromptCaching` from issue #83's sketch are not added speculatively — see Decision §3.
- **Does not wire `with_cached_system()`/`mark_ephemeral` into any real vendor request mapping** — that is ADR-068's job entirely (gated on ADR-048's `AnthropicCompleter` existing at all). This ADR's builder has no effect on any wire payload until ADR-068's mapping is built.
- **Does not change `CacheControl`'s `cache_type: String` into a closed enum**, nor add Anthropic's newer `ttl` sub-field — both are ADR-068's stated scope boundary, inherited unchanged here.

## Consequences

**What this enables**
- A real, awaitable `ToolOps::execute` — tool backends that need async I/O (web search, code execution, external API calls) can implement it without `block_on()`'s panic/blocking hazard, closing ADR-043's L1 known limitation.
- `CompletionRequest` can express a per-request timeout, giving `CompleteError::Timeout(u64)` a producer-side field to be populated from, closing L2.
- `ModelInfo`'s capability surface becomes additive going forward — a new capability is one enum variant, not a breaking struct-field addition — closing L3.
- `edge-llm-complete` gains its first real, non-test consumer of `CacheableMessage`/`cache_control`, and callers get a one-line way to mark the highest-value Anthropic caching target (the system prompt) instead of hand-assembling a `Message` with `Role::System` and calling `mark_ephemeral` themselves — closing L4.

**What this requires**
- `async-trait` crate usage extended to `ToolOps` (already a dependency, already used on `Completer`/`ModelOps` — no new dependency).
- Four `impl ToolOps` sites converted to `#[async_trait]`/`async fn execute` (`noop_completer.rs`, and three test doubles across `tool_ops_e2e_test.rs`/`tool_call_loop_e2e_test.rs`), plus the one production call site (`tool_call_step.rs:36-38`) gains `.await`.
- One cross-crate compile fix in `edge-llm-provider`: `completion_input.rs:58-72`'s exhaustive `CompletionRequest` literal needs `timeout_ms: None` added — a required follow-up in a different crate's PR, not optional.
- Two test files (`model_info_e2e_test.rs`, `model_ops_e2e_test.rs`) change from bare-field reads (`m.supports_vision`) to method calls (`m.supports_vision()`).
- `with_cached_system`'s `Result`-returning signature is a small ergonomic cost relative to the issue's original `-> Self` sketch — callers write `req.with_cached_system("...")?` instead of a bare chain — justified by not introducing a panic path in library code.
- Arch audit must remain at 183/183 for `edge-llm-complete` after all four changes, per each issue's own stated acceptance bar.

**Arch-mandate note (per this ADR's own scoping instruction):** only #81 changes an existing trait method's signature, and it was already Request/Response-shaped and `*Error`-suffixed before this ADR — the only change is adding `async`. #82, #83, and #84 are data-type/builder additions (a struct field, a struct's field shape, and two inherent methods returning `Self`/`Result<Self, _>`), not new or changed trait methods, so the mandatory Request/Response port-shape rule does not apply to them directly — there is no new trait method being introduced for any of the three.

## Alternatives Considered

**Make `ToolOps::available_tools`/`tool_choice`/`merge_delta` async too, for uniformity**
Rejected. None of the three do I/O — `available_tools`/`tool_choice` return static or derived metadata, `merge_delta` mutates an in-memory struct. Making them `async fn` returning already-available values would be async-for-async's-sake, adding executor overhead and `.await` noise at every call site (including inside `merge_delta`'s tight per-chunk streaming loop) for no correctness benefit. Only `execute` is where a real implementer needs to await something.

**Add `timeout_ms` as a required (non-`Option`) field with a default duration**
Rejected. `CompletionRequest` derives `Default`, and every other tunable (`temperature`, `max_tokens`, `top_p`, `stop`) is already `Option`-shaped so "unset" means "defer to the completer's own default" rather than baking a specific millisecond value into the domain type. A required field would also make the one exhaustive literal in `completion_input.rs` need a *value*, not just `None`, forcing `edge-llm-provider` to invent a default duration that belongs in a vendor plugin, not this port.

**Add a `.with_timeout_ms(ms)` builder method alongside the field**
Considered, not added. Every other `Option` field on `CompletionRequest` is set via `..Default::default()`/struct-update syntax with no dedicated builder; adding one only for `timeout_ms` would be inconsistent with the rest of the struct's existing convention (contrast `with_cached_system`/`with_system`, which earn their existence because they perform real logic — constructing a `Message`, calling `mark_ephemeral`, inserting at index 0 — not just setting one field).

**Keep `ModelInfo`'s three booleans and add new ones as new booleans, deferring the enum conversion**
Rejected — this is the status quo issue #83 was filed against. Every new capability (structured output, audio, extended context, parallel tool calling, prompt caching) would keep being a breaking struct-field addition. The one-time cost of the enum conversion (touching the constructor and two tests, per the verified blast radius above) is small and non-recurring; deferring it only defers a cost that gets paid again per future capability.

**Model `ModelInfo.capabilities` as `Vec<ModelCapability>` exactly as issue #83's sketch shows, instead of `HashSet`**
Rejected. `ModelCapability` already derives `Eq + Hash` in the issue's own sketch — clearly written with a set in mind — and a `Vec` permits duplicate entries and requires an O(n) linear scan in `supports()`. `HashSet` costs nothing extra given the derives are already there.

**Make `with_cached_system` panic (`.expect(...)`) instead of returning `Result`**
Rejected. `mark_ephemeral`'s trait signature is `Result`-returning; swallowing that into a panic inside a convenience builder would violate this project's "no `panic!()` in library code" rule and hide a real (if currently unreachable) error path behind an `.expect()` that would only surface as a crash in a downstream caller's process, not a typed error they can react to.

## Tracking

- `sweengineeringlabs/edge-domain#81` — `ToolOps::execute` → `async fn`; 4 `impl ToolOps` sites + 1 production call site (`tool_call_step.rs:36-38`) to update
- `sweengineeringlabs/edge-domain#82` — `CompletionRequest.timeout_ms: Option<u64>`; **required follow-up in `edge-llm-provider`**: `completion_input.rs:58-72`'s exhaustive literal needs `timeout_ms: None`
- `sweengineeringlabs/edge-domain#83` — `ModelInfo` capabilities → `HashSet<ModelCapability>` + `supports()`/`supports_vision()`/`supports_function_calling()`/`supports_streaming()` accessors; confirmed zero cross-crate consumers of `edge_llm_complete::ModelInfo`'s booleans; confirmed no conflict with ADR-056 (different `ModelInfo` type) or ADR-050 (reads `context_window` only)
- `sweengineeringlabs/edge-domain#84` — `CompletionRequest::with_cached_system()`/`with_system()`; first real (non-test) consumer of `CacheableMessage::mark_ephemeral` in this crate
- Follow-up (separate ADR, already drafted): ADR-068 — wiring `Message.cache_control` into `AnthropicCompleter`'s actual Anthropic request JSON, the real consumer this ADR's builder feeds
- Follow-up (not blocking, unscoped here): a `Completer`/vendor plugin actually enforcing `CompletionRequest.timeout_ms` as a deadline (ADR-048 territory)
- Follow-up (not blocking, unscoped here): unifying `edge_llm_complete::ModelInfo` and `edge_llm_provider::ModelInfo` (ADR-050's pre-existing tracked gap, unaffected by this ADR)
- Not tracked as new work here: adding `ModelCapability` variants beyond `Vision`/`FunctionCalling`/`Streaming` — additive and cheap whenever a real consumer needs one
