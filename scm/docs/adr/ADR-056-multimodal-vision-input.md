# ADR-056: Multimodal Vision Input for `edge-llm-provider`'s `CompletionInput`/`CompletionMessage`

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-043 (LLM Complete), ADR-045 (`edge-llm-runtime` Composition Root — names "no real multimodal input path" as an explicit unsolved gap), ADR-046 (Tool Governance — establishes the `edge_domain_policy::Policy` reuse pattern this ADR follows), ADR-048 (real vendor Completer — dependency for actually exercising this)
**GitHub Issues:** TBD

---

## Context

`ModelInfo` (`domain/scm/domain/llm/complete/main/src/api/complete/types/model_info.rs:15`) declares `pub supports_vision: bool`. A repo-wide grep of `supports_vision` turns up exactly two kinds of hits: the field/doc-comment itself, a `Default`/constructor that always sets it `false` (`complete/main/src/core/complete/model_info.rs:18`), and arch-audit test-coverage nags (`provider/docs/7-operations/compliance/structural_audit_report_20260618_134938.json`) about the accessor needing a test. **Zero call sites read the flag to make a decision.** It is a capability that is declared and never checked — a "declare and abandon" field.

**Correction (post-review):** this ADR's enforcement point (below) gates on a *different, separately-declared* `supports_vision: bool` — `edge_llm_provider::ModelInfo` (`provider/main/src/api/provider/types/model_info.rs:19`), fetched via `Provider::model_info()` — not the `edge_llm_complete::ModelInfo.supports_vision` named directly above. The two are structurally identical but genuinely distinct types with independent always-`false` constructors; gating one does not make the other load-bearing. This ADR's chosen chokepoint (`Provider::complete`, where the model is already looked up) is still the right one — `edge_llm_complete::ModelInfo` is a `Completer`-level concept, and `Completer` implementations are vendor-swappable, so checking at the `Provider` boundary avoids duplicating the gate into every future `Completer` impl (see Alternatives Considered). But the Consequences section below is corrected to name the field this ADR actually fixes; `edge_llm_complete::ModelInfo.supports_vision` remains exactly as dead as described above, now explicitly tracked under "What this ADR explicitly does NOT solve" rather than silently conflated with the field this ADR does fix. (Cross-referenced in ADR-066, which caught the same duplication from the other side.)

Before designing a fix, this ADR verified the actual shape of the content model it would need to extend, per the investigation mandate — and the finding reverses the assumption in the original problem statement:

**`MessageContent` is not bare text. It is already a proper multi-part enum, in two places:**

- `domain/scm/domain/llm/complete/main/src/api/complete/types/message_content.rs:8` — `pub enum MessageContent { Empty, Text(String), Parts(Vec<ContentPart>) }` (`#[serde(untagged)]`, `Empty` is `#[default]`).
- `domain/scm/domain/llm/agents/main/src/api/types/message_content.rs:6` — a structurally parallel, but distinct, `pub enum MessageContent { Text(String), Parts(Vec<ContentPart>) }` (no `Empty` variant).

Both crates also already define `ContentPart` with real image variants:

- `complete/main/src/api/complete/types/content_part.rs:15-33` — `ContentPart::{ Text { text }, ImageUrl { image_url: Box<ImageUrl> }, ImageBase64 { data, media_type } }`, where `ImageUrl { url: String, detail: Option<String> }` (`complete/main/src/api/complete/types/image_url.rs:8`).
- `agents/main/src/api/types/content_part.rs:5-23` — the same three variants (`Text`, `ImageUrl { image_url: String }`, `ImageBase64 { data, media_type }`), field-for-field compatible modulo the `ImageUrl`/`Box<ImageUrl>` wrapper.

**Stranger still: one real, tested image-carrying path already exists.** `agents/main/src/core/conversation/conversation_turn_step.rs` — the loop that actually drives a multi-turn agent conversation through a `Completer` — converts this crate's own `Message`/`MessageContent`/`ContentPart` into `edge_llm_complete`'s equivalents turn-by-turn (`to_complete_content`, lines 123-130; `to_complete_part`, lines 132-150), and it already maps `ContentPart::ImageUrl`/`ImageBase64` correctly, with passing tests (`test_to_complete_part_image_url_happy`, `test_to_complete_part_image_base64_edge`, lines 339-370). Images already flow, end-to-end at the type level, through the agent conversation loop into `CompletionRequest` → `Completer::complete`.

**The actual, narrow gap is one level up, in `edge-llm-provider`.** `Provider::complete` (`provider/main/src/api/provider/traits/provider.rs:74-77`) is the crate's own public "single structured completion" entry point — the one wired to a real `Completer` call in commit `78a4d8c` ("wire `CompletionInput` into a real `Provider::complete` call"), and the one any future HTTP-facing composition root (ADR-045) is documented to prefer over raw `Completer` access. Its request type, `CompletionInput` (`provider/main/src/api/provider/types/completion_input.rs`), carries `Vec<CompletionMessage>`, and:

```rust
// provider/main/src/api/provider/types/completion_message.rs:13-18
pub struct CompletionMessage {
    pub role: MessageRole,
    pub content: String,   // <- bare text, no way to attach an image
}
```

Both conversion sites hard-code the text variant, with no branch that could ever produce anything else:

```rust
// provider/main/src/core/provider/completion/completion_message.rs:32-38
pub(crate) fn into_message(self) -> Message {
    Message { role: self.role.into_role(), content: MessageContent::Text(self.content), ..Message::default() }
}
// provider/main/src/core/provider/completion/completion_input.rs:76-82
fn system_message(content: String) -> Message {
    Message { role: Role::System, content: MessageContent::Text(content), ..Message::default() }
}
```

So the fix is not "invent a multi-part content model" — it already exists and is already proven for images on one path. The fix is: thread the *existing* `edge_llm_complete::MessageContent` (already a workspace dependency of `edge-llm-provider` — `provider/Cargo.toml:29`) through `CompletionMessage`, the same way `agents`' conversion functions already thread its own parallel type, and add the one check that's genuinely missing: does the active model's `ModelInfo.supports_vision` allow this?

`CompletionMessage`'s constructors (`user`/`assistant`/`tool`) and `Provider::complete` (`std_provider.rs`) have no consumers outside `edge-llm-provider`'s own `main/src` and `tests/` — confirmed by a repo-wide grep for `CompletionMessage::` — so widening the `content` field's type is a contained, low-blast-radius change.

## Decision

### 1. Widen `CompletionMessage.content` to the existing rich type

```rust
// provider/main/src/api/provider/types/completion_message.rs
pub struct CompletionMessage {
    pub role: MessageRole,
    pub content: edge_llm_complete::MessageContent,   // was: String
}
```

`into_message` (`completion_message.rs` core) stops wrapping: `Message { content: self.content, .. }` — no `MessageContent::Text(...)` hard-coding left anywhere in this conversion.

`CompletionMessage::user`/`assistant`/`tool` keep their existing `impl Into<String>` sugar (still the common case, zero call-site churn for text-only callers) but build `MessageContent::Text(content.into())` explicitly rather than relying on the field type to coerce it. A new low-level constructor, `CompletionMessage::with_content(role: MessageRole, content: MessageContent) -> Self`, is added for callers that need `Parts([ContentPart::ImageUrl { .. }, ContentPart::Text { .. }])` — mirroring `ContentPart`'s two image shapes (URL-referenced, base64-inline) exactly as they already exist in `edge_llm_complete`. No new `ImageSource`/`ContentPart` type is introduced; the existing `edge_llm_complete::{ContentPart, ImageUrl}` are reused as-is.

`CompletionInput::system_message` is unaffected (a system prompt is definitionally text-only; left as `MessageContent::Text`).

### 2. Enforce `ModelInfo.supports_vision` at the one real chokepoint

The check belongs where the model is actually known and the request is about to leave the crate's control — `StdProvider::complete` (`provider/main/src/core/provider/std/std_provider.rs:134-150`), immediately after the existing `model_info(...)` lookup (line 138) and before `into_completion_request` runs (line 143). This is a single, fixed rule (not a pluggable per-tenant policy set, unlike ADR-046's capability/risk gates), but it is still expressed as an `edge_domain_policy::Policy` impl rather than an inline `if`, for the same reason ADR-046 gave: the port already exists, is tested, and composes for free if a future ADR needs to AND it with other completion-time rules.

```rust
// provider/main/src/api/provider/types/vision_capability_request.rs (new)
pub struct VisionCapabilityRequest {
    pub model: String,
    pub message_has_image: bool,
    pub model_supports_vision: bool,
}

// provider/main/src/core/provider/policy/vision_capability_policy.rs (new, pub(crate))
pub(crate) struct VisionCapabilityPolicy;
impl edge_domain_policy::Policy for VisionCapabilityPolicy {
    type Input = VisionCapabilityRequest;
    fn name(&self, _: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "vision-capability" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, VisionCapabilityRequest>) -> Result<(), PolicyError> {
        let input = req.input;
        if input.message_has_image && !input.model_supports_vision {
            return Err(PolicyError::new(
                "vision-capability",
                format!("model '{}' does not declare supports_vision", input.model),
            ));
        }
        Ok(())
    }
}
```

`CompletionInput` gains a `pub(crate) fn contains_image_content(&self) -> bool` helper (in `completion_input.rs`) that scans `self.messages` for any `MessageContent::Parts` containing `ContentPart::ImageUrl`/`ImageBase64` — the same pattern-match `to_complete_part` already uses in the `agents` crate, just read-only.

`StdProvider::complete` evaluates the policy and translates a `PolicyError` into the provider's own, structurally typed error — it does **not** leak `PolicyError` out of `Provider::complete`'s existing `Result<_, ExecutionError>` signature, keeping the trait's public error surface unchanged:

```rust
// ExecutionError (provider/main/src/api/provider/errors/execution_error.rs) — new variant
#[serde(rename = "vision_not_supported")]
VisionNotSupported { model: String },
```

```rust
// std_provider.rs::complete, inserted between the existing model/temperature lookups and into_completion_request
if VisionCapabilityPolicy.evaluate(PolicyEvaluateRequest { input: &VisionCapabilityRequest {
    model: model.clone(),
    message_has_image: req.input.contains_image_content(),
    model_supports_vision: /* from model_info() already fetched above */,
} }).is_err() {
    return Err(ExecutionError::VisionNotSupported { model });
}
```

### Shape / workspace layout

All additions live inside the existing `edge-llm-provider` crate — no new crate, matching the small scope of the actual gap:

```
domain/scm/domain/llm/provider/
├── main/src/api/provider/
│   ├── types/
│   │   ├── completion_message.rs        (content: String → MessageContent)
│   │   └── vision_capability_request.rs (new)
│   └── errors/errors/execution_error.rs (+ VisionNotSupported variant)
└── main/src/core/provider/
    ├── completion/
    │   ├── completion_message.rs        (into_message stops wrapping in ::Text)
    │   └── completion_input.rs          (+ contains_image_content)
    ├── policy/vision_capability_policy.rs (new, pub(crate))
    └── std/std_provider.rs              (complete() gains the gate)
```

New dependency: `edge-domain-policy` added to `provider/Cargo.toml` (it is not currently a dependency of this crate — confirmed by reading `provider/Cargo.toml`). No changes to `edge-llm-complete`'s or `edge-llm-agent`'s existing `MessageContent`/`ContentPart` — both already have everything this ADR needs; they are consumed, not modified.

## What this ADR explicitly does NOT solve

- **No real vendor `Completer` exists to actually send an image anywhere.** `EchoCompleter`/`NoopCompleter` remain the only implementations (per ADR-045's own "What this ADR explicitly does NOT solve"). This ADR makes the *port* honest — a caller of `Provider::complete` can now construct and pass image content, and get a typed error if the model can't take it — but nothing downstream does anything with the bytes until ADR-048 lands a real backend. Do not read this ADR as "vision works"; it defines the shape, ADR-048 is what exercises it.
- **Does not touch the HTTP ingress boundary.** `DefaultProviderHandler` (`provider/main/src/core/provider/default_provider_handler.rs:19-71`), the `Handler` actually registered per ADR-045, takes `Request = String` (a bare goal) and delegates to `ExecutionModel::execute_step` — a different, even-more-text-only path than `CompletionInput`. Making an image reachable from an actual HTTP request requires separate follow-on work on that handler (or a new one) — not addressed here.
- **Does not unify `agents::MessageContent`/`ContentPart` and `complete::MessageContent`/`ContentPart` into a single shared type.** They stay two structurally-parallel-but-distinct enums, bridged by `DefaultConversationTurnStep`'s hand-written `to_complete_content`/`to_complete_part` (unchanged by this ADR). Collapsing them is a larger, separate refactor with its own blast radius across both crates' public APIs — out of scope.
- **Does not touch the `supports_vision`-is-a-plain-`bool` shape question** raised in GitHub issue #83 (whether model capabilities should be a `ModelCapability` enum set). This ADR reads the existing `bool` as-is; the type-shape question is orthogonal and tracked separately (see ADR-066).
- **Does not make `edge_llm_complete::ModelInfo.supports_vision` load-bearing.** Per the Context correction above, this ADR gates `edge_llm_provider::ModelInfo.supports_vision` instead — a separate field. The `complete`-crate field named in this ADR's own Context as the motivating "declare and abandon" example remains exactly as dead after this ADR ships as before it. Unifying the two `ModelInfo` types (or wiring the `complete`-side field to something) is separate, untracked-elsewhere work.
- **No image validation** — size limits, base64 well-formedness, URL reachability/scheme allowlisting, content-type sniffing. `contains_image_content`/`VisionCapabilityPolicy` answer one question only: "does this request carry an image part, and can the model take one at all?" Nothing here inspects the image itself.
- **No per-tenant or configurable override of the vision gate.** `VisionCapabilityPolicy` is a single fixed rule, not composed via `CompositePolicy` with other rules — there is currently exactly one rule to enforce, so composition is available but not exercised. A future ADR can add more `Policy<Input = VisionCapabilityRequest>`-shaped rules (e.g., per-tenant image-size ceilings) without changing this one's shape.

## Consequences

**What this enables**
- `Provider::complete`'s public request type can express image content (URL-referenced or inline base64) using the same `ContentPart`/`ImageUrl` shapes already proven correct by the `agents` crate's conversation loop — no new content vocabulary invented.
- `edge_llm_provider::ModelInfo.supports_vision` becomes load-bearing: previously-dead data now gates a real code path and returns a typed, structured `ExecutionError::VisionNotSupported { model }` instead of silently sending an image to a model that can't use it or silently dropping it. (Note: this is `provider`'s own `ModelInfo`, not the `complete`-crate field named in Context — see the Context correction and "does NOT solve" above.)
- Consistent behavior between the two content-carrying paths in the repo: the `agents` conversation loop already refuses nothing (no gate exists there either, out of scope here — see below) but at least `Provider::complete` no longer physically cannot represent an image, closing the gap ADR-045 named.

**What this requires**
- `edge-domain-policy` added as a new dependency of `edge-llm-provider`.
- `CompletionMessage.content`'s type changes from `String` to `edge_llm_complete::MessageContent` — a breaking change to `edge-llm-provider`'s public API. Confirmed low blast radius: a repo-wide grep for `CompletionMessage::` shows call sites confined to `provider/main/src` and `provider/tests/` (no external crate or example constructs `CompletionMessage` directly).
- New tests per the mandatory `_happy`/`_error`/`_edge` scenarios: `test_contains_image_content_with_image_part_happy` / `test_complete_vision_not_supported_error` (image content + `supports_vision: false` → `ExecutionError::VisionNotSupported`) / `test_complete_text_only_model_without_vision_edge` (text-only content on a `supports_vision: false` model still succeeds — the gate must not over-trigger).
- `VisionCapabilityError` is intentionally **not** a new error type — `PolicyError` stays internal to the gate's own evaluation, and the public surface change is a new variant on the provider crate's existing, already-`*Error`-suffixed `ExecutionError`, per the project's error-naming rule.
- Still gated on ADR-048 for any real end-to-end proof; until then, this is verified with `EchoCompleter`/`NoopCompleter` only, exactly as ADR-045 scoped its own transport-plumbing proof.

## Alternatives Considered

**Invent a provider-local rich content enum instead of reusing `edge_llm_complete::MessageContent`**
Rejected. `edge-llm-provider` already depends on `edge-llm-complete` (`provider/Cargo.toml:29`), and the target type already exists, already has `Image` variants, and is already exercised correctly by the `agents` crate's conversion functions. Defining a second, provider-local content enum would duplicate a type that's one `use` away and create a third mapping function to keep in sync, on top of the one `agents`↔`complete` mapping that already exists.

**Unify `agents::MessageContent`/`ContentPart` and `complete::MessageContent`/`ContentPart` into one shared type as part of this ADR**
Rejected for this ADR's scope. It's a real simplification (it would delete `DefaultConversationTurnStep`'s ~30-line hand-mapping), but it touches both crates' public API surfaces and has nothing to do with the actual reported gap (`CompletionMessage` being unable to express an image at all). Tracked as a follow-up, not bundled here.

**Enforce the vision-capability gate inside each `Completer` implementation rather than once in `StdProvider::complete`**
Rejected. `Completer` implementations are vendor-shaped (`EchoCompleter`, `NoopCompleter` today; real vendor backends under ADR-048 tomorrow); the `ModelInfo` the check needs is a `Provider`-level concept, fetched via `Provider::model_info` — not something every `Completer` independently owns. Checking once at the `Provider::complete` boundary, where the model is already being looked up anyway (line 138), is the single source of truth; duplicating it into every future `Completer` impl would be exactly the kind of copy-paste variation this codebase's conventions reject.

**Silently drop image parts (degrade to text-only) when the model doesn't support vision, instead of erroring**
Rejected. Silently discarding part of the caller's request is a silent-swallow anti-pattern and a correctness hazard — the caller would get a completion that looks successful but never actually saw the image it sent. A typed `ExecutionError::VisionNotSupported` lets the caller react deliberately (switch model, strip the image explicitly, or surface the error to its own user).

**Inline the capability check as a plain `if` in `std_provider.rs` instead of an `edge_domain_policy::Policy` impl**
Considered, not chosen. A plain `if` would be marginally less code for a single fixed rule. Rejected in favor of consistency with ADR-046's established pattern (business-rule gates are `Policy` impls, not ad hoc conditionals) and to leave room for future composition (e.g., a per-tenant image-size ceiling) without a later rewrite.

## Tracking

- `edge-domain-policy` dependency addition to `edge-llm-provider/Cargo.toml`
- `CompletionMessage.content: String → MessageContent` (breaking change, confirmed zero external consumers)
- New `ExecutionError::VisionNotSupported { model: String }` variant
- New `VisionCapabilityPolicy` (`core/provider/policy/`) + `VisionCapabilityRequest` (`api/provider/types/`)
- New `CompletionInput::contains_image_content` helper
- Follow-up (separate ADR, not blocking): wire image content through `DefaultProviderHandler`/HTTP ingress, which today is `String`-goal-only via `ExecutionModel::execute_step`
- Follow-up (separate ADR, not blocking): unify `agents::MessageContent`/`ContentPart` with `complete`'s equivalents, deleting `DefaultConversationTurnStep`'s hand-written mapping
- Depends on ADR-048 (real vendor `Completer`) for any end-to-end proof that an image was actually sent to a model
