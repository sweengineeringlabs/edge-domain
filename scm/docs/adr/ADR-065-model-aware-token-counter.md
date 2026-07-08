# ADR-065: Model-Aware `TokenCounter`

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-034 (LLM Prompt), ADR-050 (Context-Window Guard — corrected by this ADR, see Decision), ADR-048 (Real Vendor Completer — same "prove the port before the backend" scoping pattern)
**GitHub Issues:** sweengineeringlabs/edge-domain#106

---

## Context

Issue #106 (`gh issue view 106 --repo sweengineeringlabs/edge-domain`) records a gap first noted in `llmprompt#30` and carried via `edge-domain#100`: the shipped `TokenCounter` port is **text-only**. Reading `prompt/main/src/api/prompt/traits/token_counter.rs:10-30` confirms this exactly — the trait has four methods, none of which take a model identifier:

```rust
pub trait TokenCounter: Send + Sync {
    fn count_tokens(&self, req: CountTokensRequest<'_>) -> Result<CountTokensResponse, PromptError>;
    fn estimate_tokens(&self, req: EstimateTokensRequest<'_>) -> Result<EstimateTokensResponse, PromptError>;
    fn tokenizer_name(&self, req: TokenizerNameRequest) -> Result<TokenizerNameResponse, PromptError>;
    fn is_exact(&self, req: ExactnessRequest) -> Result<ExactnessResponse, PromptError>;
}
```

`CountTokensRequest<'a>` (`count_tokens_request.rs:5-8`) and `EstimateTokensRequest<'a>` (`estimate_tokens_request.rs:5-8`) each carry a single field, `text: &'a str` — no model. `TokenizerNameRequest`/`ExactnessRequest` are unit structs (called as bare `TokenizerNameRequest`/`ExactnessRequest` in every test, e.g. `token_counter_svc_int_test.rs:92,101`). The only shipped implementation, `HeuristicTokenCounter` (`core/prompt/heuristic_token_counter.rs:40-78`), confirms the consequence: `count_tokens` and `estimate_tokens` both divide `req.text.chars().count()` by a single, construction-time `chars_per_token` ratio (`ratio()`, `heuristic_token_counter.rs:35-37`, default `4` per `DEFAULT_CHARS_PER_TOKEN`, line 13) with no branching on which model the text is destined for. `tokenizer_name` always returns the constant `"heuristic-chars"` (line 16, 66-72) and `is_exact` always returns `false` (75-77) — both regardless of model too.

### ADR-050 already assumed a `TokenCounter` call — and did not thread a model through it

ADR-050 (`docs/adr/ADR-050-context-window-guard.md`) designs `ContextWindowPolicy`, evaluated inside `StdProvider::complete()`, fed by exactly one `TokenCounter` call. Its proposed `complete()` body (ADR-050:137-139) is:

```rust
let estimated = self.token_counter.estimate_tokens(EstimateTokensRequest {
    text: &request.prompt_text(), // system + messages joined; see note below
})?.count as u32;
```

This calls `estimate_tokens` with only `text` — because that is the only field the trait has today. ADR-050 separately sources `context_window` from `ModelInfo` (`info.context_window`, ADR-050:143), so the *window size* is already model-specific, but the *token estimate* it is compared against is not: `HeuristicTokenCounter::estimate_tokens` produces the same count for `"claude-3-opus"` and `"gpt-4-turbo"` given identical text, because it never sees which model the request is headed to. Since GPT's and Claude's tokenizers diverge meaningfully (different vocabularies, different whitespace/subword handling), a guard that compares a model-blind estimate against a model-specific window is silently miscalibrated for every model whose real chars-per-token ratio differs from the single global default of `4`. **This is a real gap in ADR-050's own design, not a new, independent enhancement layered on top of it** — see Decision.

Verified this has not shipped yet, so the correction below is a pre-implementation amendment, not a patch to running code: `provider/main/src/core/provider/std/std_provider.rs:19-33` (current, on-disk) still shows the pre-ADR-050 constructor —

```rust
pub fn new(
    config: ProviderConfig,
    model: ModelInfo,
    completer: Arc<dyn Completer>,
    observer: Arc<dyn edge_domain_observer::ObserverContext>,
) -> Self {
    Self { config, model: Some(model), completer, observer }
}
```

— with no `token_counter` field, no `context_policy` field, and `complete()` (`std_provider.rs:134-150`) still calling `self.completer.complete(...)` directly with no pre-flight check. `TokenCounter` has zero non-test callers anywhere in the workspace today (`grep -r "TokenCounter"` across `domain/llm` returns only trait/type/test/SAF files — see Tracking).

### The object-safety caveat already on record

Issue #106 flags: "`context_window` was withdrawn earlier precisely because a `model`-only signature (no `&self`) isn't object-safe on a `dyn` trait. Design accordingly." This refers to `edge` ADR-034 §A (the upstream `edge-llm-prompt` ADR, hosted in the `edge` monorepo's own `docs/3-architecture/adr/`, linked from `ADR-045`'s header — not a file under this `scm/docs/adr` directory). This ADR takes the caveat as binding: any new `context_window` method must be an ordinary `&self` trait method, callable through `Arc<dyn TokenCounter>`, exactly like the four existing methods.

### Which `ModelInfo` — and does `TokenCounter` get to use it?

ADR-050 already established (ADR-050:16-18) that **two distinct `ModelInfo` types exist**:
- `edge_llm_provider::api::provider::types::ModelInfo` (`provider/main/src/api/provider/types/model_info.rs:6-30`) — `id, name, family: ModelFamily, context_window, supports_vision, supports_functions, supports_streaming, training_cutoff`. This is the one `StdProvider` holds and the one ADR-050's guard reads.
- `edge_llm_complete::api::complete::types::ModelInfo` (`complete/main/src/api/complete/types/model_info.rs:5-20`) — structurally similar (`id, name, provider, context_window, supports_vision, supports_function_calling, supports_streaming`), unrelated by any conversion, never read by `StdProvider`.

Both types, and the `ModelFamily` enum `provider::ModelInfo` embeds (`model_info.rs:14`), live in `edge-llm-provider`. ADR-050 also verified the dependency graph (ADR-050:56-62): `provider/Cargo.toml` depends on `edge-domain-handler`, `edge-domain-observer`, `swe-edge-configbuilder`, `edge-llm-complete` — and, per ADR-050's own decision, is gaining `edge-llm-prompt` too, making the graph `provider → {prompt, complete}`. `prompt/Cargo.toml` depends on `edge-domain-handler`/`edge-domain-observer` only — it does **not** depend on `provider` or `complete`, and must not start to: adding `prompt → provider` on top of the now-decided `provider → prompt` would be a direct cycle. So `TokenCounter`'s new model-aware methods **cannot** key off either `ModelInfo` type or `ModelFamily` without breaking the DAG ADR-050 just fixed in place. They take a plain `model: &str` and do their own, self-contained interpretation of it — see Decision.

## Decision

Extend `TokenCounter` in place (not a new sibling trait — see Alternatives) with model-aware counting and a new, object-safe `context_window` method, scoped for v1 to **per-model-family heuristic tuning**, not real per-vendor tokenizer libraries.

### Signature changes (Request/Response shape — fields added, not positional args)

`CountTokensRequest`/`EstimateTokensRequest` each gain a `model` field:

```rust
// count_tokens_request.rs
pub struct CountTokensRequest<'a> {
    pub text: &'a str,
    pub model: &'a str,
}

// estimate_tokens_request.rs
pub struct EstimateTokensRequest<'a> {
    pub text: &'a str,
    pub model: &'a str,
}
```

A new method + request/response pair, mirroring the existing `TokenizerNameRequest`/`Response` shape:

```rust
// context_window_request.rs
pub struct ContextWindowRequest<'a> {
    pub model: &'a str,
}

// context_window_response.rs
pub struct ContextWindowResponse {
    pub context_window: u32,
}
```

```rust
pub trait TokenCounter: Send + Sync {
    fn count_tokens(&self, req: CountTokensRequest<'_>) -> Result<CountTokensResponse, PromptError>;
    fn estimate_tokens(&self, req: EstimateTokensRequest<'_>) -> Result<EstimateTokensResponse, PromptError>;
    fn tokenizer_name(&self, req: TokenizerNameRequest) -> Result<TokenizerNameResponse, PromptError>;
    fn is_exact(&self, req: ExactnessRequest) -> Result<ExactnessResponse, PromptError>;
    /// Approximate context window for `model`, as this counter's own heuristic
    /// understands it — NOT authoritative; see "What this ADR explicitly does
    /// NOT solve".
    fn context_window(&self, req: ContextWindowRequest<'_>) -> Result<ContextWindowResponse, PromptError>;
}
```

`context_window` takes `&self` (satisfying the ADR-034 §A object-safety caveat) so `Arc<dyn TokenCounter>` remains constructible and usable exactly as it is today.

### `HeuristicTokenCounter`'s model-awareness: string-prefix family detection, not a `ModelInfo`/`ModelFamily` lookup

Since `prompt` cannot depend on `provider` (see Context), `HeuristicTokenCounter` classifies `model: &str` by plain prefix/substring matching against known model-name conventions (`"claude"`, `"gpt-4"`, `"gpt-3.5"`, `"gemini"`, etc.) into a small internal ratio table, entirely local to the `prompt` crate:

```rust
fn ratio_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();
    match () {
        _ if m.contains("claude") => 4,   // Anthropic: ~4 chars/token, English prose
        _ if m.contains("gpt-4") || m.contains("gpt-3.5") => 4, // cl100k_base ballpark
        _ if m.contains("gemini") => 4,
        _ => Self::DEFAULT_CHARS_PER_TOKEN, // unrecognized model: conservative default, not an error
    }
}
```

(Illustrative ratios above are placeholders for the implementation PR to calibrate against real tokenizer output samples per family — the point of this ADR is the *port shape and dependency boundary*, not the exact constant.) Unrecognized model strings fall back to the existing global default rather than returning `PromptError` — no new error variant is introduced; `PromptError::TokenizationError(String)` (`errors/prompt_error.rs:56-57`) already exists if a future implementation needs to signal a hard failure, but the heuristic counter's contract stays "always answers, degrades to a conservative default," matching its existing never-fails behavior (`is_exact` always returns `Ok`).

`context_window(&self, ContextWindowRequest { model })` returns a small static per-family table of known context sizes inside `prompt` — deliberately **not** sourced from `edge_llm_provider::ModelInfo`. This value is a self-contained fallback for callers that only have a model string and no `ModelInfo` in scope; it is not a replacement for the authoritative figure `ContextWindowPolicy` already reads.

### Required correction to ADR-050

Because ADR-050 is `Proposed` and unimplemented (Context, above), this is a pre-implementation amendment to ADR-050's own snippet, not a follow-up patch to shipped code. ADR-050:137-139's proposed `complete()` body must become:

```rust
let estimated = self.token_counter.estimate_tokens(EstimateTokensRequest {
    text: &request.prompt_text(),
    model: &model, // `model` is already in scope two lines up: info.id.clone()
})?.count as u32;
```

`ContextWindowCheckInput` (ADR-050's new type, `context_window_check_input.rs`) and its `context_window: u32` field are **unchanged** — that value still comes from `info.context_window` (`ModelInfo`, ADR-050:143), not from `TokenCounter::context_window()`. The two figures are intentionally allowed to exist side by side: `ModelInfo.context_window` (provider-owned, authoritative, used by the guard) and `TokenCounter::context_window()` (prompt-owned, heuristic, for callers with no `ModelInfo`). `ContextWindowPolicy` must keep reading the former; this ADR does not redirect it to the latter — doing so would trade a known-authoritative source for a heuristic one for no benefit, and would reintroduce the `prompt`↔`provider` coupling question ADR-050 just resolved one way.

### Scope: heuristic tuning per model family, not real per-vendor tokenizers

Two shapes were possible for "model-aware":
(a) Keep the existing character/word-count heuristic, but vary its ratio by model family (small, local, no new dependencies).
(b) Delegate to real per-vendor tokenizer libraries — a tiktoken-equivalent BPE implementation for GPT, Anthropic's own tokenizer for Claude, etc.

This ADR chooses **(a) for v1**. This mirrors ADR-048's explicit scoping discipline for real vendor completers: "One vendor, fully real ... is a stronger deliverable than N vendors partially real" (`ADR-048-real-vendor-completer.md:98`). The equivalent claim here: one heuristic, tuned and tested per known model family, fully wired through `ContextWindowPolicy`, is a stronger and shippable v1 than a partial multi-vendor real-tokenizer integration that would require vetting and vendoring at least two separate third-party tokenizer crates (with their own licensing, binary-size, and update-cadence considerations) before any of it is usable. Real per-vendor tokenizers are an explicit, larger follow-up (see What this ADR explicitly does NOT solve / Tracking), consistent with this series' established pattern of proving the port shape before proving a real backend (ADR-048's Anthropic-first Completer, ADR-069's OpenAI-second Completer).

### Call sites requiring updates

`TokenCounter` has no non-test consumers yet (ADR-050's consumer is proposed but unbuilt — see Context), so every call site needing an update is either the trait/impl itself or a test:

- `prompt/main/src/api/prompt/traits/token_counter.rs` — trait signature (add `context_window`)
- `prompt/main/src/api/prompt/types/count_tokens_request.rs`, `estimate_tokens_request.rs` — add `model` field
- new: `prompt/main/src/api/prompt/types/context_window_request.rs`, `context_window_response.rs`
- `prompt/main/src/core/prompt/heuristic_token_counter.rs` — `impl TokenCounter` (lines 40-78) gains model-keyed ratio dispatch + `context_window`; its own `#[cfg(test)]` module (lines 94-141) constructs `CountTokensRequest`/`EstimateTokensRequest` with only `text` at lines 97, 106, 110, 122 — each needs a `model: "..."` literal added
- `prompt/tests/token_counter_svc_int_test.rs` — SAF-facade tests construct bare `CountTokensRequest`/`EstimateTokensRequest` at lines 19-21, 31, 42-44, 48, 60, 70, 80 — all need a `model` field
- `prompt/tests/token_counter_e2e_test.rs` — lines 14, 24 — same
- `prompt/tests/heuristic_token_counter_int_test.rs` — lines 13-15, 24, 34 — same
- `prompt/tests/prompt_factory_svc_int_test.rs` — lines 211, 223 (`@covers: PromptBootstrap::token_counter`) — same
- `docs/adr/ADR-050-context-window-guard.md:137-139` — amend the proposed snippet as shown above, **before** ADR-050 is implemented

## What this ADR explicitly does NOT solve

- **Real per-vendor tokenizers.** No tiktoken-equivalent BPE for GPT, no Anthropic tokenizer integration. The heuristic remains a heuristic; `is_exact` continues to report `false` for every model. Tracked as a separate, larger follow-up ADR (see Tracking).
- **`tokenizer_name()`/`is_exact()` remain model-unaware.** Once `count_tokens`/`estimate_tokens` vary by model family, a single counter instance can report a different effective ratio per model but still only one `tokenizer_name` ("heuristic-chars") and one `is_exact` (`false`) regardless of which model was just counted. This is a real, if minor, inconsistency this ADR does not fix — flagged as a known gap, not silently resolved.
- **Does not fix the two-`ModelInfo`-types duplication** (`edge_llm_provider::ModelInfo` vs `edge_llm_complete::ModelInfo`) — still ADR-050's tracked, separate cleanup, untouched here.
- **Does not redirect `ContextWindowPolicy` to `TokenCounter::context_window()`.** The guard's authoritative context-window source stays `ModelInfo.context_window` per ADR-050, unchanged by this ADR. `TokenCounter::context_window()` is a self-contained fallback for `ModelInfo`-less callers, not a second implementation of the same fact.
- **Does not add a `prompt → provider` (or `prompt → complete`) dependency.** Model classification inside `HeuristicTokenCounter` is plain string matching, deliberately not a `ModelFamily`/`ModelInfo` lookup, to keep the DAG ADR-050 established intact.
- **Does not change `PromptError`.** No new variant; unrecognized models degrade to the existing default ratio/window rather than erroring.
- **Does not touch `edge-llm-provider`'s `StdProvider::complete()` directly** — that wiring is ADR-050's, corrected in place per Decision above; this ADR only changes the `TokenCounter` port and its one reference implementation in `edge-llm-prompt`.

## Consequences

**What this enables**
- `TokenCounter::count_tokens`/`estimate_tokens` finally vary by destination model, closing issue #106 via option (a) (per the issue's own framing: "(a) provision a model-aware counter here, or (b) leave token-counting external. This issue tracks option (a)").
- ADR-050's `ContextWindowPolicy`, once built, compares a per-family-tuned estimate against a per-model window instead of a single flat heuristic against a per-model window — closing the calibration gap identified in Context, before that guard ever ships.
- `llmprompt#27`'s external `tokenizer` crate dependency becomes droppable per the issue's own "Consumers" note, once a model-aware in-tree counter exists.
- Establishes a documented boundary (`TokenCounter::context_window()` = heuristic fallback vs. `ModelInfo.context_window` = authoritative) that any future caller of either can rely on instead of guessing which is "the real one."

**What this requires**
- Breaking change to `CountTokensRequest`/`EstimateTokensRequest` (new required field) — six files updated (impl + 4 test files + the factory test file), enumerated above.
- Two new type files (`context_window_request.rs`, `context_window_response.rs`) and one new trait method, with `_happy`/`_error`/`_edge` scenario tests per this repo's mandatory pattern (unknown-model fallback is the natural `_edge` case; a recognized family per known model is `_happy`).
- `saf/prompt/token/token_counter_svc.rs` re-export (`pub use crate::api::TokenCounter;`) needs no change — it re-exports the trait itself, not its methods individually.
- ADR-050's own document must be amended (see Decision) before its implementation PR lands — sequencing note, not a code change in this repo state.
- `arch audit --rs` re-run for `edge-llm-prompt` after the new method/types land — expected to stay at parity (no new orphan types: `ContextWindowRequest`/`Response` are used in the `TokenCounter::context_window` signature immediately).

## Alternatives Considered

**New sibling trait (`ModelAwareTokenCounter`) instead of extending `TokenCounter` in place**
Rejected. `TokenCounter` has exactly one implementation (`HeuristicTokenCounter`) and zero non-test consumers today (ADR-050's consumer is proposed, not built) — there is no shipped caller whose contract this would break, and no reason to fork the port into two parallel traits a caller would have to choose between. Extending in place is strictly cheaper here and avoids a second port for the same concept.

**`context_window` as a bare associated function keyed only on `model` (no `&self`)**
Rejected outright per issue #106's own citation of `edge` ADR-034 §A: this exact shape was tried before and withdrawn because it is not object-safe on `dyn TokenCounter`. This ADR keeps `&self` on every method, no exceptions.

**Have `TokenCounter::context_window()` read `edge_llm_provider::ModelInfo` (or `edge_llm_complete::ModelInfo`) directly**
Rejected. Would require `edge-llm-prompt` to depend on `edge-llm-provider` and/or `edge-llm-complete`. ADR-050 already fixed the graph as `provider → {prompt, complete}` (ADR-050:56-62); adding an edge back from `prompt` would create a cycle with `provider` and gives `prompt` a dependency on whichever of the two duplicate `ModelInfo` types it picked — inheriting that unresolved duplication rather than staying clear of it.

**Real per-vendor tokenizers now (tiktoken-equivalent + Claude tokenizer) instead of a tuned heuristic**
Rejected for v1, for the same reason ADR-048 scoped to one vendor rather than N: bigger surface (two or more third-party tokenizer dependencies, their own licensing/binary-size/update-cadence considerations, and per-vendor accuracy validation work) with no proof yet that the port shape (`model` field on `Count`/`EstimateTokensRequest`, `&self`-based `context_window`) is right. Prove the port with a cheap, in-tree heuristic first; swap in real tokenizers behind the same trait later without another signature change.

**Leave `TokenCounter` text-only and resolve model-awareness entirely in the caller (e.g. `ContextWindowPolicy` picks a ratio itself)**
Rejected — this is issue #106's option (b), which the issue text says would be closed as wontfix in favor of `llmprompt#27`'s external crate. Pushing model-awareness into every caller duplicates the same family-detection logic anywhere token counting is needed (today just `ContextWindowPolicy`, but `edge-llm-prompt`'s own `ContextManager`/prompt-building paths are equally plausible future callers) instead of centralizing it once behind the port that already exists for exactly this purpose.

## Tracking

- `prompt/main/src/api/prompt/traits/token_counter.rs` — add `context_window` method
- `prompt/main/src/api/prompt/types/count_tokens_request.rs`, `estimate_tokens_request.rs` — add `model` field
- New: `prompt/main/src/api/prompt/types/context_window_request.rs`, `context_window_response.rs`
- `prompt/main/src/core/prompt/heuristic_token_counter.rs` — per-family ratio dispatch + `context_window` impl + inline test updates
- Call-site updates: `prompt/tests/token_counter_svc_int_test.rs`, `token_counter_e2e_test.rs`, `heuristic_token_counter_int_test.rs`, `prompt_factory_svc_int_test.rs`
- **`docs/adr/ADR-050-context-window-guard.md`** — amend the `complete()` snippet (ADR-050:137-139) to pass `model` into `EstimateTokensRequest`, before ADR-050's implementation PR lands; no change to `ContextWindowCheckInput` itself
- Issue #106 — closes option (a); comment back with the object-safety confirmation (`&self` retained) and the DAG-safety confirmation (no `prompt → provider`/`complete` edge added)
- `llmprompt#27` — external consumer; can drop the standalone `tokenizer` crate once this ships, per issue #106's "Consumers" note
- Follow-up (separate, larger ADR, not blocking this one): real per-vendor tokenizer integration (tiktoken-equivalent for GPT family, Anthropic's tokenizer for Claude family) behind the same `TokenCounter` port
- Not tracked here, separate cleanup (per ADR-050, unchanged): unify `edge_llm_provider::ModelInfo` and `edge_llm_complete::ModelInfo`
