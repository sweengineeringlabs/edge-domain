# ADR-064: `edge-llm-prompt` — Public Surface Encapsulation Fix

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-034 (LLM Prompt Domain), ADR-045 (`edge-llm-runtime` Composition Root), ADR-046 (`edge-llm-tools` Governance)
**GitHub Issues:** [sweengineeringlabs/edge-domain#103](https://github.com/sweengineeringlabs/edge-domain/issues/103) — `fix(prompt): encapsulation.package_access_violation — lib.rs leaks 52 concrete types (→183/183)`, part of Epic #97, gated by #100 (ADR-034 finalization)

---

## Context

Issue #103 (filed against Epic #97, "the full LLM-landscape audit") describes `edge-llm-prompt`'s `main/src/lib.rs` as re-exporting 52 concrete implementation types via `pub use saf::*` — a wildcard re-export that lets every type behind `saf/` (`HeuristicTokenCounter`, `MapContextManager`, `PromptCache`, `StaticPrompt`, and roughly 48 others) leak onto the public crate boundary, in violation of the arch rule named in the issue title, `encapsulation.package_access_violation`, and SEA Rule 47 ("only trait contracts, factories, and value types may cross the boundary").

**That specific problem, as described, no longer exists in the code.** Reading `main/src/lib.rs` in full today (lines 1–48) shows no `pub use saf::*` wildcard anywhere — the crate already uses four explicit, named `pub use` blocks:

- `main/src/lib.rs:19-22` — 4 `saf`-re-exported traits + 5 `*_SVC` markers
- `main/src/lib.rs:23-26` — 5 `*_SVC_FACTORY` markers
- `main/src/lib.rs:29-34` — 17 concrete/value types re-exported directly from `api::`
- `main/src/lib.rs:37-47` — 30 Request/Response envelope types, also from `api::`

Running the audit live (`~/.cargo/bin/arch audit --rs` from `domain/scm/domain/llm/prompt`, 2026-07-08) confirms this. The pipeline is gated (`project [PASS] → package [PASS] → api [1 violation, halts here] → core/spi/saf/gateway [not assessed]`), and within the `api` gate's `encapsulation` section, `encapsulation.package_access_violation` itself shows **`[PASS]`**. The one violation that halts the run is a different, more specific rule: `no_orphan_types`, with **7 offenders**, all in `main/src/api/prompt/types/`:

```
PromptCache, PromptCacheBuilder, PromptMetadata, PromptMetadataBuilder,
PromptTemplateBuilder, StdPromptFactory, VariableBuilder
```

Net result: **169/172 passed, 1 failed, 2 skipped** — not the 182/183 → 183/183 the issue's acceptance criteria assume, because the ruleset itself has moved on since the issue was filed (consistent with the gated-cascade behavior already documented in ADR-045's Context and this repo's `reference_arch_layer_gating_and_rpitit` notes: the denominator and even the specific rule catalog shift as the arch tool version and upstream gates change).

Two things had to be established before writing a Decision, and both are now verified against real code, not assumed:

**1. Is the current `api::` re-export list (47 non-trait, non-marker names) actually a `package_access_violation`-style problem, comparable to what the issue described?**
No. Cross-checking against the two other LLM crates already fully audited on this exact rule — `edge-llm-provider` (`main/src/lib.rs:17-38`) and `edge-llm-agent` (`main/src/lib.rs:33-65`) — both use the **identical shape**: a `saf::{...}` block (traits + `*_SVC`/`*_SVC_FACTORY` markers) plus one or more `api::{...}` blocks carrying concrete reference-implementation types (`StdProvider`, `EchoExecutionModel`, `EchoProviderCompleter` in provider; `NoopAgent`, `NoopSkill`, `BoundedConversationLoop` in agents), plain value/vocabulary types, and Request/Response envelopes — all re-exported directly from `api::`, not funneled through `saf/`. `saf/mod.rs` in both `prompt` (`main/src/saf/mod.rs:1-10`) and `provider` (`main/src/saf/mod.rs:1-9`) contains **only** trait names and `*_SVC`/`*_SVC_FACTORY` consts — zero concrete types. `encapsulation.package_access_violation`'s own rule text (`arch explain encapsulation.package_access_violation`) confirms what it actually checks: that `saf/` doesn't re-export concrete types and that `lib.rs`'s `pub use saf::{...}` brace list contains only trait names — it does not restrict what `lib.rs` separately re-exports from `api::`. `edge-llm-prompt`'s current `lib.rs` conforms to the same, already-audited pattern as its two siblings. This is not prompt inventing a workaround; it's the established, working convention across the crate family.

**2. Is the residual `no_orphan_types` failure (7 offenders) prompt-specific technical debt, or the same accepted tradeoff already carried elsewhere?**
The same accepted tradeoff, quantitatively worse. Live audits of the sibling crates show the identical failure shape:

| Crate | `no_orphan_types` offenders | Result |
|---|---|---|
| `edge-llm-provider` | 2 (`EchoProviderCompleter`, `StdProviderFactory`) | 169/172, 1 failed |
| `edge-llm-agent` | 2 (`ConversationState`, `ConversationTurnStep`) | 169/172, 1 failed |
| `edge-llm-prompt` | 7 (listed above) | 169/172, 1 failed |

Every one of prompt's 7 offenders already carries an in-source `// Orphan-type note:` doc comment explaining, case by case, why it cannot be wired into a trait method signature without inventing ceremony:

- `main/src/api/prompt/types/prompt_cache.rs:3-6` — `Prompt::cache` returns `CacheBuildResponse` (flattened fields, per SEA `field_type_purity`), never `PromptCache` itself; the type still has a real internal consumer (`main/src/core/prompt/static_prompt.rs:12,16`, `main/src/core/prompt/std_prompt_factory.rs`) but never crosses a trait boundary directly.
- `main/src/api/prompt/types/prompt_cache_builder.rs:3-5` — plain fluent builder for the above; builders are never trait method parameters.
- `main/src/api/prompt/types/prompt_metadata.rs:5-8` — same rationale: `Prompt::metadata` returns `PromptMetadataResponse` (`main/src/api/prompt/types/prompt_metadata_response.rs:5-9`, explicitly documented as "flattened from `PromptMetadata`"); the struct is still live internally (`main/src/core/prompt/default_prompt_handler.rs:80`, `main/src/core/prompt/prompt_template.rs:20-21`, `main/src/core/prompt/std_prompt_factory.rs:21,27`).
- `main/src/api/prompt/types/prompt_metadata_builder.rs:5-8` — builder for the above.
- `main/src/api/prompt/types/prompt_template_builder.rs:6-9` — builder for `PromptTemplate`, which itself is **not** orphaned (referenced by-reference inside live Response types: `main/src/api/prompt/types/template_lookup_response.rs:9`, `list_templates_response.rs:9`, `list_by_category_response.rs:9`, and by-value inside `main/src/api/prompt/types/catalog_template_provider.rs:13`).
- `main/src/api/prompt/types/variable_builder.rs:5-8` — builder for `Variable`, which is likewise not orphaned (`main/src/api/prompt/types/register_variable_request.rs:11` takes `&'a Variable` directly).
- `main/src/api/prompt/types/std_prompt_factory.rs:3-8` — the factory exposes its behavior via inherent methods returning `impl Handler`, never by implementing a trait itself; the exact same shape as provider's `StdProviderFactory` offender.

This matches the compositional pattern the audit tool itself already exempts elsewhere: `CatalogTemplateProvider`, `HeuristicTokenCounter`, `MapContextManager`, and `StaticPrompt` are **not** on the offender list, because each genuinely implements one of the four port traits (`main/src/core/prompt/catalog_template_provider.rs:45` `impl TemplateProvider for CatalogTemplateProvider`, `heuristic_token_counter.rs:40` `impl TokenCounter for HeuristicTokenCounter`, `map_context_manager.rs:28` `impl ContextManager for MapContextManager`, `static_prompt.rs:50` `impl Prompt for StaticPrompt`) and are exempted under the rule's documented "concrete trait implementor" carve-out (`arch explain no_orphan_types`). The 7 real offenders are qualitatively different: plain data/builder/factory types with no trait to implement, already reasoned through and documented at the point they were introduced (git blame traces the doc comments to `6a72186 fix(llm-prompt): convert all traits to Request/Response envelope pattern`).

## Decision

Categorize `lib.rs`'s ~52-type public surface by role — not by enumerating all 52 individually — and confirm each category is already correctly placed:

| Category | Count | Examples | Disposition |
|---|---|---|---|
| Trait contracts (via `saf/`) | 4 | `ContextManager`, `Prompt`, `TemplateProvider`, `TokenCounter` | Correctly public. Unaffected. |
| SAF service/factory markers | 10 | `*_SVC`, `*_SVC_FACTORY` consts | Correctly public — these are the composition-root's lookup keys (ADR-045's registration pattern), not implementation. Unaffected. |
| Request/Response envelopes | 30 | `RenderRequest`, `CacheBuildResponse`, `VariableLookupRequest`, … | Correctly public — callers cannot invoke a single trait method without constructing/reading these (`api_method_takes_request`/`api_response_type_named` mandate them). Unaffected. |
| Plain value/vocabulary types | 6 | `JsonValue`, `PromptError`, `PromptTemplate`, `RenderContext`, `Variable`, `VariableKind` | Correctly public — each is referenced live, by value or reference, inside a Request/Response field that a trait method actually uses (verified per-type above). Unaffected. |
| Concrete reference implementations | 4 | `CatalogTemplateProvider`, `HeuristicTokenCounter`, `MapContextManager`, `StaticPrompt` | Correctly public — each is the crate's one shipped `impl <Trait> for Self`, the identical shape as provider's `Echo*`/agents' `Noop*` types. Unaffected. |
| Orphaned-by-design plain data/builder/factory types | 7 | `PromptCache`, `PromptCacheBuilder`, `PromptMetadata`, `PromptMetadataBuilder`, `PromptTemplateBuilder`, `StdPromptFactory`, `VariableBuilder` | Pre-existing, individually documented tradeoff (see Context) — same class of exemption already carried by `edge-llm-provider` (2) and `edge-llm-agent` (2). Not re-litigated by this ADR. |

Consequently:

1. **Close issue #103 as resolved-by-prior-work, not as a fresh remediation.** The rule it names (`encapsulation.package_access_violation`) already passes; the `pub use saf::*` wildcard it describes is already gone, replaced with the explicit, sibling-consistent named-export shape. No `lib.rs` edit is required to fix the rule the issue actually names.
2. **Do not force `no_orphan_types` to 0 offenders by re-architecting `PromptCache`/`PromptMetadata`/the builders/`StdPromptFactory` into fake trait parameters.** All 7 are plain data or builder or factory types with no natural trait method that would take or return them by name; inventing one (e.g. a `CacheBuilder` trait whose only method is `build`) would be ceremony with no real polymorphism — exactly the "force a port that doesn't belong" anti-pattern this series of ADRs has been careful to avoid (see ADR-045/046's own restraint around not inventing unneeded abstractions). Each offender already carries this reasoning in-source; this ADR ratifies it as the accepted position rather than silently accumulating it as unexplained debt.
3. **Update the issue's acceptance criteria to match the current, real rule catalog.** "183/183" was written against an older arch tool/rule version. The crate family's actual, achievable ceiling under the current tool is 169/172 (provider and agents already sit there too), gated on this one documented, cross-crate-consistent `no_orphan_types` exemption class.
4. **No new Request/Response trait method is introduced by this ADR.** This is a structural/encapsulation classification exercise, not a port-design change — consistent with the task framing that a raw encapsulation fix, unlike ADR-045/046, has no new domain contract to design. `Prompt`, `ContextManager`, `TemplateProvider`, and `TokenCounter`'s signatures are untouched.

## What this ADR explicitly does NOT solve

- It does not reduce `edge-llm-prompt`'s public surface below what `edge-llm-provider`/`edge-llm-agent` already carry — parity with the established sibling pattern is the target, not a stricter bar invented for this crate alone.
- It does not touch `no_orphan_types` in `edge-llm-provider` or `edge-llm-agent` — their 2-offender counts are out of scope here and already covered by the same "verified tradeoff" reasoning.
- It does not address ADR-034's still-open finalization gate (#100) that issue #103 was gated behind; that gate concerns the prompt domain's public contract shape generally, not this specific encapsulation question.
- It does not change `saf/mod.rs`, `main/src/lib.rs`'s module declarations (`mod api; mod core; mod saf; mod spi;`), or any trait signature in `api/prompt/traits/`.
- It does not attempt to make the `arch` tool itself recognize plain-data/builder/factory types as a first-class exemption category (the way it already recognizes "concrete trait implementor"); that would be a change to the audit tool, not to this crate, and is called out under Alternatives Considered as a rejected-for-now option.

## Consequences

**What this enables**
- Issue #103 can be closed against reality: the rule it names is verifiably passing today, with the exact `lib.rs` line ranges cited above as evidence, not a re-run assumption.
- The crate family (`provider`, `agent`, `prompt`) now has one documented, cross-referenced rationale for the shared `no_orphan_types` tradeoff class, instead of three separate, independently-argued justifications that could drift out of sync.
- Future audits of `edge-llm-reasoning`/`edge-llm-complete` (not covered by this ADR) have a template: check whether their own `no_orphan_types` offenders are the same plain-data/builder/factory shape before assuming they need a code fix.

**What this requires**
- No source code changes to `edge-llm-prompt`.
- Updating issue #103's acceptance criteria (183/183 → the crate's actual achievable ceiling, 169/172, matching provider/agent) or closing it with this ADR as the resolution record.
- Communicating to whoever owns Epic #97 that the epic's "182/183 → 183/183" framing predates a rule-catalog change and should be re-baselined crate-by-crate rather than assumed uniform.

**Risks of not doing this**
- Without this ADR, issue #103 remains open indefinitely against a rule that already passes, and a future contributor could "fix" it by literally deleting `PromptCache`/`PromptMetadata`/the builders — which would break `main/src/core/prompt/static_prompt.rs`, `std_prompt_factory.rs`, `prompt_template.rs`, and `default_prompt_handler.rs`, all of which use these types as real internal domain values, plus the five `tests/*_int_test.rs` files that exercise them as part of the crate's public "client library" surface (`catalog_template_provider_int_test.rs`, `prompt_cache_int_test.rs`, `prompt_cache_builder_int_test.rs`, `prompt_metadata_builder_int_test.rs`, `variable_builder_int_test.rs`, `prompt_template_builder_int_test.rs`).

## Alternatives Considered

**Force `no_orphan_types` to 0 by inventing a marker trait per offender (e.g. `CacheBuildable`, `MetadataBuildable`)**
Rejected. This is exactly the anti-pattern the in-source doc comments already argue against ("ceremony with no real polymorphism"). It would satisfy the checker while adding trait surface nobody calls polymorphically — a `TODO`-driven-development smell in reverse (code added purely to please a linter, not to serve a real caller).

**Delete the 7 orphaned types and their builders outright**
Rejected. Verified false: these are not dead code. `PromptCache`, `PromptMetadata`, `PromptTemplate` (built by `PromptTemplateBuilder`), and `Variable` (built by `VariableBuilder`) all have live internal consumers in `core/prompt/` (cited above), and the builders are the crate's documented "client library" ergonomics surface, exercised by five external integration tests. Deleting them would be a regression, not a fix.

**Route the 47 `api::`-sourced re-exports through `saf/` instead of direct `pub use api::{...}` in `lib.rs`**
Rejected. This would diverge from the established, already-audited convention in `edge-llm-provider` and `edge-llm-agent`, which both re-export concrete/value/envelope types directly from `api::` alongside the `saf::` trait block. Making `prompt` the only crate in the family funneling everything through `saf/` would be inconsistency for its own sake, not a rule requirement — `encapsulation.package_access_violation` only constrains what `saf/` itself re-exports and what `lib.rs`'s `saf::{...}` brace list contains, not separate `api::{...}` blocks.

**Leave issue #103 open, unexamined, assuming the 52-type/183-target framing is still accurate**
Rejected. Running the live audit is what surfaced that the named rule already passes and that a different rule (`no_orphan_types`) is the actual, smaller, already-family-wide-accepted gap. Leaving the issue as originally filed would misdirect future work at a rule that isn't failing.

## Tracking

- Close or re-scope `sweengineeringlabs/edge-domain#103`: rule named in the title (`package_access_violation`) already passes; re-file remaining scope (if any) against `no_orphan_types` with corrected acceptance criteria (169/172, matching `edge-llm-provider`/`edge-llm-agent`), or close outright citing this ADR.
- Update Epic #97's tracked target for `edge-llm-prompt` from "183/183" to the tool's current achievable ceiling for this crate (169/172), with a note that the ceiling number is itself a moving target tied to the arch tool's rule catalog, not a fixed constant.
- No new prerequisite work, no new crate, no new trait — this ADR is a classification and documentation record, not an implementation ADR.
