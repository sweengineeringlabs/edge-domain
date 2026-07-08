# ADR-063: `TemplateRenderer` + `ContextConfig`/`WarningLevel` — Closing Two of ADR-034's Never-Built Concepts

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** [edge ADR-034](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-034-llm-prompt.md) (LLM Prompt — being finalized), ADR-052 (`edge-llm-retrieval` — the other recent consumer of `ContextManager`), ADR-046 (`edge-llm-tools` — established the `edge_domain_policy::Policy`-reuse pattern this ADR tests and rejects for a different shape)
**GitHub Issues:** `sweengineeringlabs/edge-domain#101` (analysis this ADR closes the design portion of), `#105` (ticketed, out of scope here — see below), `#106` (ticketed, out of scope here — see below)

---

## Context

ADR-034 §4/§5 (`docs/3-architecture/adr/ADR-034-llm-prompt.md:130-169`) specified a `TemplateRenderer` trait (`render`/`extract_variables`/`validate_variables`) and a `ContextConfig`/`WarningLevel` threshold model, both under `edge_llm_prompt`'s own `api/traits/` and `api/types/` (ADR-034 Architecture section, lines 220 and 228-229). Neither was ever built. `sweengineeringlabs/edge-domain#101` tracks this, alongside the separate, larger question of ADR-034 §2's capacity/pruning `ContextManager` (`add_item`/`prune`/`snapshot`/`Priority`/`PruneStrategy`).

**What #101's own three comments already settled, so this ADR does not re-litigate it:**

1. Comment 1 found that a *window-trim* `ContextManager` (`manage()`/`sliding_window()`/`trim_messages()`) plus a `ContextConfig`/`WarningLevel` pair matching ADR-034 §5 field-for-field already exists in `swelabs/llmprompt/scm/context` — a **different, promotable** design from §2's priority-pruning model.
2. Comment 2 (self-correction) confirmed that promotable design does **not** implement §2's `add_item`/`prune(PruneStrategy)`/`snapshot`/`Priority`/`ContextItem`/`ContextSnapshot` — that model stays **withdrawn** per the ADR-034 finalize amendment (`ADR-034-llm-prompt.md:344-351`, amendment §C: *"That priority-pruning model was never built anywhere; it is hereby withdrawn."*).
3. Comment 3 ticketed the **build-by-promotion of the window-trim `ContextManager`** as a separate issue, **`#105`**, dependent on a model-aware `TokenCounter`, **`#106`**. Comment 3 is explicit: *"This thread [#101] is the analysis/decision; #105 is the code."*

So `#101` already drew the line between "analysis" and "code," and already routed the mutating window-trim mechanism to `#105`. This ADR is the design for the two pieces of ADR-034 that `#101`'s comments did **not** route anywhere else: `TemplateRenderer` (never mentioned again after comment 1) and the non-mutating half of the `ContextConfig`/`WarningLevel` model — the classification logic a future `#105` will need as a building block, but which does not itself require `#105`'s `Message`/`TokenCounter` rewiring to build today.

**Verifying `TemplateRenderer` is a distinct concept from what already ships.** Two things already exist that could be mistaken for it:

- `TemplateProvider` (`domain/scm/domain/llm/prompt/main/src/api/prompt/traits/template_provider.rs:10-28`) is a **catalog**: `get_template`/`list_templates`/`list_by_category` — lookup and enumeration of registered templates. It has no `render` method and never touches variable substitution. Confirmed distinct, as `#101` asserts.
- `StaticPrompt::substitute` (`domain/scm/domain/llm/prompt/main/src/core/prompt/static_prompt.rs:27-46`) is the current, minimal templating baseline: for each `Variable` **already declared in `PromptMetadata`**, it builds the literal token `"{{name}}"` and calls `output.replace(&token, &value.display())` (line 39). Two gaps this leaves, both closed by ADR-034's `TemplateRenderer` spec, not by this method:
  - It never scans the template body itself for placeholders — it only iterates *declared* variables. A typo'd placeholder (`{{nam}}` in the template body vs. a declared `Variable{name: "name"}`) produces no error and no substitution: the literal token `"{{name}}"` is never found in `output`, `context.get_variable("name")` still returns `Some`, so the loop silently does nothing and `render()` reports success with the typo left in the output. Nothing today parses raw template text for the placeholders it actually contains.
  - `substitute` is `pub(crate)` — internal to one `Prompt` impl. There is no reusable, standalone, testable primitive for "given a template string and a variable map, render it" independent of `PromptMetadata`/`RenderContext`/a concrete `Prompt`.

ADR-034 §4's `TemplateRenderer` (`render(&self, template: &str, variables: &[(&str, &str)])`, `extract_variables(&self, template: &str) -> Vec<String>`, `validate_variables(...)`) is exactly the missing piece: a template-first (not metadata-first) primitive that can catch the drift `substitute` cannot, because it actually parses the template text for the placeholders it contains rather than trusting a separately-maintained declaration list.

**Checking reuse before designing `ContextConfig`/`WarningLevel`'s mechanism.** Per this codebase's established discipline (ADR-046 replaced ADR-036's bespoke `GovernancePolicy` with `edge_domain_policy::Policy` because a threshold/gate check is exactly policy-shaped), the same check is owed here. `edge_domain_policy::Policy` (`domain/scm/domain-policy/main/src/api/policy/traits/policy.rs:13-24`) is:

```rust
pub trait Policy: Send + Sync {
    type Input;
    fn name(&self, req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError>;
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, Self::Input>) -> Result<(), PolicyError>;
}
```

`evaluate` is strictly binary: `Ok(())` (satisfied) or `Err(PolicyError)` (`domain/scm/domain-policy/main/src/api/policy/errors/policy_error.rs:19-25`, `{ policy: &'static str, reason: String }`) — no data channel back to the caller beyond a name and a reason string. `WarningLevel` (`None`/`Warning`/`Critical`, ADR-034 line 164-168) is a **three-valued classification**, and two of its three values (`None`, `Warning`) are *not* failures — a caller needs to distinguish "usage is fine" from "usage is high but not yet critical" to decide whether to log at info vs. warn level, or surface an approaching-limit signal, none of which is a policy violation. Forcing `Warning` down the `Err(PolicyError)` channel would misrepresent a non-failure, informational outcome as a rule breach, and every caller would have to catch-and-ignore an "error" that isn't one. `Policy` is the right shape for ADR-046's capability/risk gates (pass/fail, deny on violation); it is the wrong shape for a graded, informational classification whose job is to hand back *which* of three levels applies, not whether one rule passed.

## Decision

Add `TemplateRenderer` and `ContextConfig`/`WarningLevel` (+ a small classification trait, `ContextCapacityMonitor`) directly to the existing `edge-llm-prompt` crate (`domain/scm/domain/llm/prompt/`), exactly where ADR-034's own Architecture section placed them (`template_renderer.rs`, `context_config.rs`, `warning_level.rs` — `ADR-034-llm-prompt.md:220,228-229`). No new crate — this is ADR-034's own crate finishing two of its five original concepts, the same way `edge-llm-retrieval` (ADR-052) added a *new* crate for a concept ADR-034 never named at all. Reuses the crate's 9-variant `PromptError` (`.../api/prompt/errors/prompt_error.rs`) for every new fallible path; no new error type.

### Shape

**1. `TemplateRenderer` — the raw-template mechanics primitive.**

```rust
// api/traits/template_renderer.rs
pub trait TemplateRenderer: Send + Sync {
    /// Render `template`, substituting every `{{name}}` token found in the
    /// template body (not a pre-declared list) with `variables[name]`.
    fn render(&self, req: TemplateRenderRequest<'_>) -> Result<TemplateRenderResponse, PromptError>;

    /// Scan `template` and return every distinct `{{name}}` placeholder it
    /// actually contains, in first-occurrence order.
    fn extract_variables(&self, req: ExtractVariablesRequest<'_>) -> Result<ExtractVariablesResponse, PromptError>;

    /// Ok(()) iff every placeholder `extract_variables` would return for
    /// `template` has an entry in `variables`.
    fn validate_variables(&self, req: ValidateVariablesRequest<'_>) -> Result<(), PromptError>;
}
```

```rust
// api/types/template_render_request.rs / template_render_response.rs
pub struct TemplateRenderRequest<'a> {
    pub template: &'a str,
    pub variables: &'a HashMap<String, String>,
}
pub struct TemplateRenderResponse {
    pub rendered: String,
}

// api/types/extract_variables_request.rs / extract_variables_response.rs
pub struct ExtractVariablesRequest<'a> {
    pub template: &'a str,
}
pub struct ExtractVariablesResponse {
    pub variables: Vec<String>,
}

// api/types/validate_variables_request.rs
pub struct ValidateVariablesRequest<'a> {
    pub template: &'a str,
    pub variables: &'a HashMap<String, String>,
}
```

`validate_variables` returns bare `Result<(), PromptError>`, not a `*Response` type, on the same precedent already used in this crate for void-success methods (`ContextManager::register_variable`/`clear`, `Prompt::validate` — `.../traits/context_manager.rs:14,29`, `.../core/prompt/static_prompt.rs:75`). On failure it returns the existing `PromptError::IncompleteContext { missing_variables: Vec<String> }` variant (already used by `static_prompt.rs:55-57` for the same "these names had no value" condition) populated with every placeholder `extract_variables` found that `variables` does not cover — not the single-name `MissingVariable` variant, since more than one placeholder can be missing at once.

`variables` is a flat `HashMap<String, String>`, mirroring `ContextBuildResponse::metadata`'s `HashMap<String, String>` shape (`.../types/context_build_response.rs:15`) rather than ADR-034's literal `&[(&str, &str)]` slice-of-tuples — same map semantics, no ordering guarantee needed for a lookup-only structure, and it satisfies `field_type_purity` the same flat-map way `ContextBuildResponse` already does.

**Concrete impl:** `StdTemplateRenderer` (`core/prompt/std_template_renderer.rs`), naming-consistent with this crate's existing `Std`/`Map`/`Heuristic` concrete types (`StdPromptFactory`, `MapContextManager`, `HeuristicTokenCounter` — ADR-034 amendment §A "Layout"). It implements `{{name}}` brace-syntax extraction (scans for `{{`/`}}` pairs, collects distinct inner names) and substitution, and is the first place in this crate that actually parses template text for its placeholders rather than trusting a separately maintained `Variable` list.

**Relationship to `Prompt`/`StaticPrompt::substitute` — wraps, does not supersede, and is not merged in immediately.** `Prompt` stays the typed orchestration contract (`RenderContext.variables: HashMap<String, JsonValue>`, `Variable.required`/`.default`, `metadata()`/`validate()`/`cache()`); `TemplateRenderer` is a lower, untyped, string-only mechanics layer underneath it — the same split as `edge-llm-retrieval`'s `Embedder`/`VectorStore` staying below `ContextManager::register_variable` (ADR-052) rather than merging into it. `StaticPrompt::substitute` (`static_prompt.rs:27-46`) is left **unchanged** by this ADR: it is `pub(crate)`, already tested, and rewriting it to delegate to `StdTemplateRenderer` in the same change that introduces `StdTemplateRenderer` would mean shipping and refactoring a shipped type in one step. Unifying the two — so `StaticPrompt` calls `StdTemplateRenderer` instead of its own private loop, closing its own typo-drift gap — is called out explicitly in Tracking as a non-blocking follow-up, the same posture ADR-045 took with wrapping the provider handler in `TimeoutHandler` ("left as an explicit follow-up rather than bundled into the first version, to keep this ADR's first cut minimal," ADR-045 line 33).

**2. `ContextConfig` + `WarningLevel` — adopted verbatim from ADR-034 §5, classification only.**

```rust
// api/types/context_config.rs — unchanged from ADR-034 §5 (ADR-034-llm-prompt.md:155-162)
pub struct ContextConfig {
    pub reserved_output_tokens: u32,
    pub trim_threshold: f32,
    pub warning_threshold: f32,
    pub summarize_threshold: f32,
    pub min_retained_messages: usize,
    pub max_trim_count: usize,
}

// api/types/warning_level.rs — unchanged from ADR-034 §5
pub enum WarningLevel {
    None,
    Warning,
    Critical,
}
```

Per `#101` comment 2, these two types "match ADR-034 §5 exactly" already in the promotable `llmprompt` design, so they are adopted as-is, not redesigned.

**`ContextConfig`/`WarningLevel` are not, by themselves, a "mechanism"** — ADR-034 §5 defines the data; §2/the withdrawn model and `#105`'s window-trim promotion define what *acts* on it. This ADR adds the one piece of "acting on it" that is genuinely buildable today without `#105`'s `Message`/model-aware-`TokenCounter` rewiring: a pure classification of a token-usage count against a `ContextConfig`, with no mutation of any message list.

```rust
// api/traits/context_capacity_monitor.rs
pub trait ContextCapacityMonitor: Send + Sync {
    /// Classify `used_tokens` against `capacity_tokens` per `config`'s
    /// thresholds. Pure: reads counts, returns a level, touches no state.
    fn classify(&self, req: ContextUsageRequest<'_>) -> Result<ContextUsageResponse, PromptError>;
}

// api/types/context_usage_request.rs / context_usage_response.rs
pub struct ContextUsageRequest<'a> {
    pub config: &'a ContextConfig,
    pub used_tokens: u32,
    pub capacity_tokens: u32,
}
pub struct ContextUsageResponse {
    pub level: WarningLevel,
    pub usage_ratio: f32,
}
```

`classify` computes `usage_ratio = used_tokens / capacity_tokens`, then `Critical` if `usage_ratio >= config.trim_threshold`, else `Warning` if `>= config.warning_threshold`, else `None`. `capacity_tokens == 0` returns `PromptError::InvalidValue { variable_name: "capacity_tokens".into(), reason: "must be greater than zero".into() }` (reusing the existing variant, `.../errors/prompt_error.rs:33-38`) rather than dividing by zero.

**Concrete impl:** `StdContextCapacityMonitor` (`core/prompt/std_context_capacity_monitor.rs`), same `Std*`-naming convention as `StdTemplateRenderer`/`StdPromptFactory`.

**Why not `Policy`-shaped, restated as a decision (not just a rejected alternative):** the Context section above shows `Policy::evaluate` cannot carry `WarningLevel`'s non-failure states without misusing its error channel. There's a second, more decisive reason this ADR does not even attempt a `Policy`-shaped compromise: the part of ADR-034 §5 that would actually need pass/fail gating semantics — auto-trimming once `Critical` is reached — is the *mutating* half, and it is entirely out of scope here (routed to `#105`, see below). What's in scope is the classification only, which has no gate to enforce and nothing to mutate — so there is no pass/fail decision for `Policy` to make in the first place. A future `#105` may well have `Policy` on its own list once it defines the trim *action*; that is `#105`'s design question, not this one's.

### Workspace layout (additions to existing `edge-llm-prompt`)

```
domain/scm/domain/llm/prompt/main/src/
├── api/prompt/
│   ├── traits/
│   │   ├── template_renderer.rs              (NEW)
│   │   ├── context_capacity_monitor.rs       (NEW)
│   │   └── ... (prompt.rs, context_manager.rs, token_counter.rs, template_provider.rs — unchanged)
│   ├── types/
│   │   ├── template_render_request.rs        (NEW)
│   │   ├── template_render_response.rs       (NEW)
│   │   ├── extract_variables_request.rs      (NEW)
│   │   ├── extract_variables_response.rs     (NEW)
│   │   ├── validate_variables_request.rs     (NEW)
│   │   ├── context_config.rs                 (NEW)
│   │   ├── warning_level.rs                  (NEW)
│   │   ├── context_usage_request.rs          (NEW)
│   │   ├── context_usage_response.rs         (NEW)
│   │   └── ... (unchanged)
│   └── errors/prompt_error.rs                (unchanged — reused as-is)
├── core/prompt/
│   ├── std_template_renderer.rs              (NEW)
│   ├── std_context_capacity_monitor.rs       (NEW)
│   └── ... (static_prompt.rs unchanged — see Tracking follow-up)
└── saf/prompt/
    ├── template/template_renderer_svc.rs             (NEW — same "template" subdir as template_provider_svc.rs)
    ├── template/template_renderer_svc_factory.rs     (NEW)
    ├── context/context_capacity_monitor_svc.rs       (NEW — same "context" subdir as context_manager_svc.rs)
    └── context/context_capacity_monitor_svc_factory.rs  (NEW)
```

Both new `saf/` entries slot into the shared-prefix subdirs (`template/`, `context/`) this crate already uses for `TemplateProvider` and `ContextManager` respectively — no new subdir, no new grouping convention.

## What this ADR explicitly does NOT solve

- **The mutating window-trim `ContextManager`** (`manage()`/`sliding_window()`/`trim_messages()`, promoted from `swelabs/llmprompt/scm/context`) — that is `#105`, dependent on `#106`'s model-aware `TokenCounter`. This ADR ships the `ContextConfig`/`WarningLevel` data types `#105` will consume and a pure `classify()` it can call, but not the trim operation itself, and not the `llmprovider::Message`→`edge-llm-complete::Message` rewiring `#105` will need.
- **ADR-034 §2's priority-pruning model** (`add_item`/`prune(PruneStrategy)`/`snapshot`/`Priority`/`ContextItem`/`ContextSnapshot`) — stays withdrawn per the ADR-034 finalize amendment; nothing here resurrects it, and nothing here needs it.
- **The `ContextManager` naming collision.** The existing variable-registration `ContextManager` (`.../traits/context_manager.rs:10-33`) is untouched. This ADR does not introduce anything named `ContextManager`, so it creates no collision requiring a rename to `VariableContext`; that rename is coupled to `#105` promoting the *capacity* manager under the `ContextManager` name, not to anything added here.
- **`TokenCounter` changes.** `ContextCapacityMonitor::classify` takes plain `u32` token counts, not a `TokenCounter` call — model-aware counting is `#106`'s scope, unaffected by this ADR.
- **Unifying `StaticPrompt::substitute` with `StdTemplateRenderer`.** Both mechanisms coexist after this ADR; collapsing them is a named, non-blocking follow-up (Tracking), not bundled in here.
- **Any change to `Prompt`, `ContextManager`, `TokenCounter`, or `TemplateProvider`'s existing trait signatures.** All four are untouched.

## Consequences

**What this enables**
- A real, standalone, testable template-substitution primitive (`TemplateRenderer`) that parses a template's actual placeholders instead of trusting a separately-maintained `Variable` declaration list — closing a genuine correctness gap (`{{nam}}`-typo-style silent no-ops) `StaticPrompt::substitute` cannot catch today.
- `ContextConfig`/`WarningLevel`, specified by ADR-034 §5 and confirmed by `#101` to already have a matching promotable design, land in `edge-llm-prompt` now rather than waiting on `#105`/`#106`'s larger `Message`/`TokenCounter` rewiring.
- A pure `ContextCapacityMonitor::classify` any caller (a reasoning loop, a `Handler`) can use today to detect "approaching context limit" and log/alert via `ObserverContext` (the same seam ADR-044/ADR-046 already wired), without waiting for auto-trim to exist.
- `#105`, once picked up, has `ContextConfig`/`WarningLevel` and a reference classification already built and tested to consume, rather than having to design them from scratch alongside the harder `Message`-rewiring work.

**What this requires**
- New files only, inside the existing `edge-llm-prompt` crate (`domain/scm/domain/llm/prompt/`) — no new crate, no new workspace member.
- No changes to `Prompt`, `ContextManager`, `TokenCounter`, `TemplateProvider`, or `PromptError`'s existing variants (all 9 are reused as-is).
- `StdTemplateRenderer`/`StdContextCapacityMonitor` each need `_happy`/`_error`/`_edge` test coverage per this repo's test-scenario mandate — in particular `extract_variables` on a template with zero/duplicate/malformed placeholders, and `classify` at each threshold boundary (`usage_ratio` exactly equal to `warning_threshold`/`trim_threshold`, and `capacity_tokens == 0`).

## Alternatives Considered

**Make `ContextCapacityMonitor::classify` an `edge_domain_policy::Policy<Input = ContextUsageRequest>` impl, the way ADR-046 did for capability/risk gates**
Rejected. `Policy::evaluate` returns `Result<(), PolicyError>` with no data channel for *which* level applies; `WarningLevel::None`/`Warning` are non-failure outcomes a caller must distinguish, which the error channel cannot represent without misusing it as a smuggled return value. `Policy` is the right reuse for ADR-046's allow/deny gates; it is the wrong shape for a graded classification with no gate to enforce in this ADR's scope (the one operation that would need pass/fail — auto-trim — is deferred to `#105`).

**Refactor `StaticPrompt::substitute` to delegate to `StdTemplateRenderer` in this same ADR**
Rejected for the first cut. `StaticPrompt` is shipped and tested; folding a refactor of it into the same change that introduces the new primitive it would delegate to conflates "add a new capability" with "migrate an existing one onto it." Tracked as an explicit non-blocking follow-up instead, the same posture ADR-045 took with `TimeoutHandler`.

**Build the full ADR-034 §2 priority-pruning model (`add_item`/`prune`/`Priority`/`PruneStrategy`) now, alongside `TemplateRenderer`**
Rejected. `#101`'s own finalize-amendment (`ADR-034-llm-prompt.md:344-351`) already withdrew this model as never-built-anywhere and explicitly says to revisit only if a real consumer needs item-level pruning; no such consumer has appeared. Reopening a withdrawn model inside an ADR scoped to two different, already-open concepts would blur what `#101`'s comments deliberately separated.

**Build `#105`'s window-trim `ContextManager` (`manage()`/`sliding_window()`/`trim_messages()`) now too, since its `ContextConfig`/`WarningLevel` overlap with this ADR's**
Rejected. `#105` requires rewiring `llmprompt::Message`/`tokenizer::TokenCounter` to `edge-llm-complete::Message`/this crate's `TokenCounter` (`#101` comment 2's explicit caveat: "not a verbatim lift") and depends on `#106`'s model-aware `TokenCounter` — both are larger, separately-tracked, and already-ticketed pieces of work this ADR's title does not cover. Shipping `ContextConfig`/`WarningLevel`/`classify` now does not block `#105` from building the trim mechanism on top of them later.

## Tracking

- Land in existing crate: `edge-llm-prompt` (`domain/scm/domain/llm/prompt/`) — no new crate
- Non-blocking follow-up: refactor `StaticPrompt::substitute` to delegate to `StdTemplateRenderer`, closing `StaticPrompt`'s own typo-drift gap
- `#105` (already ticketed, unaffected by this ADR): window-trim `ContextManager` promotion from `swelabs/llmprompt/scm/context`, incl. the `ContextManager`/`VariableContext` rename this ADR does not need
- `#106` (already ticketed, unaffected by this ADR): model-aware `TokenCounter`, `#105`'s prerequisite
- Withdrawn, not reopened here: ADR-034 §2's `add_item`/`prune`/`snapshot`/`Priority`/`PruneStrategy` priority-pruning model
