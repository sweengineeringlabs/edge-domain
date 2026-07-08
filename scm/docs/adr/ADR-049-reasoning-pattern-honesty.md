# ADR-049: `edge-llm-reasoning` Pattern Honesty — Real Reflection & Tree-of-Thought First, Rest Explicitly Deferred

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** [edge ADR-035](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-035-llm-reasoning.md) (LLM Reasoning — being amended/honored by this ADR), ADR-043 (LLM Complete), ADR-045 (`edge-llm-runtime` Composition Root), ADR-046 (`edge-llm-tools` Governance), ADR-048 (Real Vendor `Completer` — Anthropic)
**GitHub Issues:** TBD

---

## Context

ADR-045's landscape audit (2026-07-08) flagged, and this ADR investigates in full: *"`edge-llm-reasoning`'s 'multi-strategy reasoning' is one linear loop cloned N times, not distinct pattern behavior"* (`ADR-045-edge-llm-runtime-standalone-composition-root.md:67`).

### The current (fake) state, confirmed by re-reading the code

- `domain/scm/domain/llm/reasoning/main/src/core/reasoning/linear_reasoning.rs` is the **only** implementer of the `Reasoning` trait (`api/reasoning/traits/reasoning.rs:19`). Its `reason()`:
  - Builds `let steps = vec![step; req.pattern.expected_step_count() as usize]` (`linear_reasoning.rs:55`) — one generic `DefaultReasoningStep` cloned N times, where N is looked up from `ReasoningPattern::expected_step_count()` (`core/reasoning/reasoning_pattern.rs:38-48`, values 2–8 per pattern).
  - Runs those N identical clones through a real `edge_pipeline::PipelineSvc` (`linear_reasoning.rs:57-69`) — the *pipeline execution* is genuine (per ADR-035's 2026-07-04 amendment, `ADR-035-llm-reasoning.md:258-270`), but every step in the pipeline does the same thing.
  - `next_step()` (`linear_reasoning.rs:114-123`) always returns `format!("next step for: {problem}")` — the pattern (`self.pattern` / `req.pattern`) is read nowhere in this method. There is no `match self.pattern` anywhere in the file that changes reasoning *behavior* — only step *count* varies by pattern (`expected_step_count()`), and `PatternMetadata::derive_max_depth()` (`core/reasoning/pattern/pattern_metadata.rs:19-22`) which is just `expected_step_count() * 2`, an unconsumed number.
  - Confirmed by the crate's own tests: `test_reason_step_count_matches_non_default_pattern_happy` (`linear_reasoning.rs:222-233`) asserts step *count* differs by pattern (2 for `FewShot`) — no test anywhere asserts step *content* differs by pattern, because it doesn't.
- A second, independently-discovered bug in the same file family: `DefaultReasoningHandler::execute` (`core/reasoning/default_reasoning_handler.rs:42-70`) always calls `self.reasoner.reason(ReasonRequest { problem: &req.req, pattern: REASONING_DEFAULT_PATTERN })` (line 62), where `REASONING_DEFAULT_PATTERN` is the hardcoded constant `ReasoningPattern::ChainOfThought` (line 19) — **regardless of which pattern the handler was constructed for**. `DefaultReasoningHandler::with_pattern(pattern)` (lines 73-79) correctly binds `LinearReasoning::new(pattern)` as the reasoner, but `execute()` never reads that binding back; it always asks the bound reasoner to run `ChainOfThought`. Since `Reasoning::reason()` rejects a `req.pattern` that isn't in `supported_patterns()` (`api/reasoning/traits/reasoning.rs:32-41`, and `LinearReasoning::supported_patterns()` returns `vec![self.pattern]`, `linear_reasoning.rs:76-83`), **`DefaultReasoningHandler::with_pattern(p)` for any `p != ChainOfThought` is broken today** — every `execute()` call would fail with `ReasoningError::UnsupportedPattern`. No test exercises `with_pattern` with a non-default pattern through `execute()`, which is why this has gone unnoticed. Pattern selection has never been exercised end-to-end for anything but the default.
- `api/reasoning/types/reasoning_pattern.rs:4-33` — `ReasoningPattern` is a 7-variant **bare unit enum** (`ChainOfThought`, `TreeOfThought`, `Reflection`, `FewShot`, `MultiAgent`, `Hierarchical`, `GraphBased`), `Copy`/`Eq`/`Hash`, no per-variant fields at all.

### What ADR-035 originally promised, vs. what exists

[edge ADR-035](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-035-llm-reasoning.md) (2026-06-16) specified a **different 7-variant set with per-variant configuration fields** (`ADR-035-llm-reasoning.md:33-77`): `CoT{max_steps,step_timeout}`, `ReAct{max_iterations,tool_timeout}`, `PlanExecute{max_plan_steps,max_exec_steps}`, `Reflexion{max_iterations,reflection_period}`, `TreeOfThought{max_depth,breadth_factor,max_nodes}`, `SelfConsistency{num_chains,aggregation}`, `LeastToMost{max_depth}`.

Comparing name-for-name against the actual `ReasoningPattern` enum:

| ADR-035 (promised) | Code (actual) | Match? |
|---|---|---|
| `CoT{max_steps,step_timeout}` | `ChainOfThought` (bare) | Name renamed, config fields dropped |
| `TreeOfThought{max_depth,breadth_factor,max_nodes}` | `TreeOfThought` (bare) | Name matches, config fields dropped |
| `Reflexion{max_iterations,reflection_period}` | `Reflection` (bare) | Name renamed (Reflexion→Reflection), config fields dropped |
| `ReAct{max_iterations,tool_timeout}` | — | **No analog in code** |
| `PlanExecute{max_plan_steps,max_exec_steps}` | `Hierarchical` (bare, loose analog at best) | Renamed/replaced, config fields dropped |
| `SelfConsistency{num_chains,aggregation}` | — | **No analog in code** |
| `LeastToMost{max_depth}` | — | **No analog in code** |
| — | `FewShot` (bare) | **No analog in ADR-035** |
| — | `MultiAgent` (bare) | **No analog in ADR-035** |
| — | `GraphBased` (bare) | **No analog in ADR-035** |

Neither the pattern *names* nor the "configuration lives in the variant" *shape* ADR-035 specified survived into code, and this divergence was never documented — ADR-035 has two prior amendments (2026-06-18 connection contract, 2026-07-04 real-`Pipeline` execution, `ADR-035-llm-reasoning.md:246-270`) but neither mentions the enum was renamed/reshaped. This ADR resolves that gap (see Decision).

### Reusable, already-proven concurrency primitive

`domain/scm/domain/llm/complete/main/src/core/complete/bounded_tool_call_loop.rs` — a different crate's multi-turn loop — is genuinely real: on each turn, it fans out one `edge_pipeline::Step` per tool call concurrently via `edge_pipeline::ParallelStepSvc` (`bounded_tool_call_loop.rs:67-92`), collecting results into a `pub(super) struct DefaultToolCallBatch` (`complete/main/src/core/complete/tool_call_batch.rs:15-41`) before continuing the loop. This is a proven fan-out/branching primitive that real tree-of-thought reasoning can reuse directly, instead of inventing new branching machinery.

`edge_llm_complete::Completer` (`complete/main/src/api/complete/traits/completer.rs:18-46`) is the port that actually talks to a model (`async fn complete(&self, req: CompleteRequest<'_>) -> Result<CompletionResponse, CompleteError>`). Neither pattern implementation proposed below can be real without calling it — reasoning today has no dependency on `edge-llm-complete` at all (`reasoning/Cargo.toml:20-26` lists only `edge-domain-handler`, `edge-domain-observer`, `edge-pipeline`). Checking the reverse direction (`complete/Cargo.toml:20-26`) confirms `edge-llm-complete` depends only on `edge-pipeline` — adding `edge-llm-reasoning → edge-llm-complete` is acyclic, and is the same shape as the already-existing `edge-llm-agent → edge-llm-complete` edge (`agents/Cargo.toml:31`).

Per ADR-048 (Real Vendor `Completer`), every `Completer` in the workspace today is still `EchoCompleter`/`NoopCompleter`/`EchoProviderCompleter` — no real vendor backend exists yet. Anything built here runs against echo/noop until ADR-048 lands; this ADR proves the **reasoning-side plumbing** (real second call, real concurrent branches), not model quality — the same scoping discipline ADR-045 used for the composition root.

`edge_domain_observer` is already a dependency of this crate and already threaded through `DefaultReasoningHandler::execute`'s span (`default_reasoning_handler.rs:46-57`, per ADR-044's pattern) — pattern-selection decisions should emit through that existing seam, not silently.

## Decision

Scope is deliberately narrow: **two** of the seven patterns get genuinely distinct, real behavior in this ADR — **Reflection** and **Tree-of-Thought** — chosen because (a) Reflection is the cheapest possible "real" implementation (one extra completion call) and (b) Tree-of-Thought's fan-out need is already solved by `ParallelStepSvc`, proven in `bounded_tool_call_loop.rs`. The other five patterns are formally scoped, not silently left implying they work.

### 1. Fix the pattern-binding bug first (independent of everything else)

`DefaultReasoningHandler` gains a `pattern: ReasoningPattern` field set consistently with `reasoner` in `with_pattern()`, and `execute()` passes `self.pattern` instead of the hardcoded `REASONING_DEFAULT_PATTERN` constant:

```rust
pub(crate) struct DefaultReasoningHandler {
    pub(crate) reasoner: Arc<dyn Reasoning>,
    pub(crate) pattern: ReasoningPattern,
}
// execute(): self.reasoner.reason(ReasonRequest { problem: &req.req, pattern: self.pattern })
```

This is a small, mechanical correctness fix that stands on its own — without it, no reasoner bound to anything but `ChainOfThought` can ever run through the handler, which would silently defeat the rest of this ADR.

### 2. `ReflectiveReasoning` — real Reflection (`core/reasoning/reflective_reasoning.rs`, new)

A second `Reasoning` implementer, bound to exactly `ReasoningPattern::Reflection`, holding `Arc<dyn edge_llm_complete::Completer>`. `reason()`:

1. **First pass** — one real `Completer::complete()` call, messages = `[Message{role: User, content: problem}]`, producing an initial answer.
2. **Second pass — the actual self-critique call** — a *second, distinct* `Completer::complete()` call, messages = `[User(problem), Assistant(first_pass_answer), User("Critique the previous answer for errors or gaps, then give an improved final answer.")]`, producing a refined answer.
3. Both passes become real `ReasoningStep`s (not template clones) appended to the `ThinkingProcess`; the conclusion is derived from the refined (second) pass, not the first.

No new error variant is needed: `CompleteError` from a failed `Completer::complete()` call maps to the **existing** `ReasoningError::StepFailed { step, reason }` (`api/reasoning/errors/reasoning_error.rs:13-20`) — `step: 0` for a first-pass failure, `step: 1` for a critique-pass failure — reusing what's already there instead of adding a fourth "unknown/failed" concept to the enum.

### 3. `BranchingReasoning` — real Tree-of-Thought (`core/reasoning/branching_reasoning.rs`, new)

A second `Reasoning` implementer, bound to exactly `ReasoningPattern::TreeOfThought`, reusing `edge_pipeline::ParallelStepSvc` in exactly the shape `bounded_tool_call_loop.rs:84-92` already proves:

- Fan-out width = `ReasoningPattern::TreeOfThought.expected_step_count()` (already `7`, `core/reasoning/reasoning_pattern.rs:41`) — reused as the branch count instead of inventing a new config knob.
- Each branch is a `Step<Ctx = DefaultReasoningBranchBatch, ExecutionError = ReasoningError>` (mirroring `DefaultToolCallStep`, referenced at `bounded_tool_call_loop.rs:74-80`) that makes one real `Completer::complete()` call, with the branch index folded into the prompt (e.g. "Explore approach #{index} to: {problem}") to encourage divergent completions across the concurrent branches — this is what makes it genuinely a *tree* exploration rather than 7 copies of the same call.
- A new `pub(crate) struct DefaultReasoningBranchBatch` (`core/reasoning/reasoning_branch_batch.rs`, new) collects each branch's resulting `ReasoningStep`s, mirroring `DefaultToolCallBatch` (`complete/main/src/core/complete/tool_call_batch.rs:15-41`).
- After the parallel step completes, the existing `build_chain()` method already on `Reasoning` (`linear_reasoning.rs:140-151`) assembles the branches into a `ReasoningChain` — reused as-is, no new aggregation type.
- **Branch selection is mandatory, not optional**: the highest-confidence branch (via the existing `evaluate_step()`/`StepResult` machinery, `linear_reasoning.rs:125-138`) is chosen as the process's conclusion. Silently concatenating all seven branches without picking one would still be "fake" tree-of-thought — exploring branches without ever selecting among them isn't the pattern's claim.

### 4. `PatternDispatchReasoning` — router (`core/reasoning/pattern_dispatch_reasoning.rs`, new)

A third `Reasoning` implementer that composes the other three (`reflective: Arc<ReflectiveReasoning>`, `branching: Arc<BranchingReasoning>`, `fallback: Arc<LinearReasoning>`) and dispatches `reason()`/`next_step()`/`evaluate_step()` by matching `req.pattern`:

```rust
match req.pattern {
    ReasoningPattern::Reflection   => self.reflective.reason(req).await,
    ReasoningPattern::TreeOfThought => self.branching.reason(req).await,
    _                               => self.fallback.reason(req).await, // honest baseline, unchanged
}
```

`supported_patterns()` reports all 7 (the router accepts every pattern; only two get real behavior, the rest get the honest `LinearReasoning` baseline they already have today — no regression, no new fakery). This becomes the reasoner `DefaultReasoningHandler` binds to by default, replacing the bare `LinearReasoning` in `with_pattern()`'s construction (item 1's fix makes this safe, since `execute()` now passes through whatever pattern the caller actually wants).

Pattern selection is made observable through the **existing** span already opened in `DefaultReasoningHandler::execute` (`default_reasoning_handler.rs:46-57`): add a span attribute `reasoning.pattern.selected = <pattern.as_str()>` and, once available, a counter `reasoning_pattern_dispatched_total{pattern=...}` — the same `ObserverContext` seam ADR-044/046 already established, not a new one.

### Shape / workspace layout

```
domain/scm/domain/llm/reasoning/
├── Cargo.toml                                  (+ edge-llm-complete.workspace = true)
└── main/src/
    ├── api/reasoning/
    │   └── errors/reasoning_error.rs            (UNCHANGED — StepFailed{step,reason} reused, no new variant)
    ├── core/reasoning/
    │   ├── linear_reasoning.rs                  (UNCHANGED — stays the honest ChainOfThought + deferred-pattern baseline)
    │   ├── reflective_reasoning.rs              (NEW — real Reflection, 2 real Completer calls)
    │   ├── branching_reasoning.rs               (NEW — real TreeOfThought via ParallelStepSvc, 7 real Completer calls)
    │   ├── reasoning_branch_batch.rs            (NEW — pub(crate) Ctx type, mirrors complete's DefaultToolCallBatch)
    │   ├── pattern_dispatch_reasoning.rs        (NEW — router Reasoning impl, observer-instrumented selection)
    │   └── default_reasoning_handler.rs         (FIXED — `pattern` field replaces hardcoded constant; binds PatternDispatchReasoning by default)
    └── saf/                                      (UNCHANGED — saf/ still re-exports only the Reasoning/Handler traits)
```

Depends on (new edge): `edge-llm-complete` (`Completer`, `CompleteError`, `CompletionRequest`, `Message`/`Role`/`MessageContent`) — verified acyclic against `edge-llm-complete`'s own dependency list.

### Resolving the ADR-035-vs-code pattern-set divergence

**Recommendation: amend ADR-035 in place to document the current 7-variant bare-enum shape as the real, accepted pattern set.** Do not reconcile the code to ADR-035's original per-variant-field design. Reasons:

1. ADR-035's per-variant fields (`max_steps`, `breadth_factor`, `num_chains`, `reflection_period`, …) were **never wired to anything**, even in the original ADR-035 sketch — no code path ever read them. Adding them now, unimplemented, would recreate the exact "declared but not wired" (`Declare-and-abandon`) anti-pattern this whole ADR exists to close, just relocated from "reasoning is fake" to "config fields are fake."
2. The equivalent configuration already exists, just as a **companion value type instead of enum payload**: `PatternMetadata` (`core/reasoning/pattern/pattern_metadata.rs:5-49`) carries `max_depth`, `max_tokens`, `min_confidence`, `allow_backtracking`, `timeout_secs`, `tags`, all derived from the bare `ReasoningPattern` via `pattern_metadata()` (already on the `Reasoning` trait, `linear_reasoning.rs:85-100`). This is arguably a *better* fit for the arch-mandated Request/Response port style than inline enum-variant fields — a bare `Copy`/`Eq`/`Hash` enum is cheaper to move through `HashMap`/`Vec::contains` (as `supports_pattern` already does, `reasoning.rs:32-41`) than a struct-carrying-variant would be.
3. `ReAct`, `PlanExecute`, `SelfConsistency`, `LeastToMost` have **no code analog at all** — reconciling would mean building four brand-new pattern names from scratch, which is strictly larger scope than this ADR's already-deliberately-narrow two-pattern target.

Action: add a third amendment section to `ADR-035-llm-reasoning.md`, in the same format as its existing two amendments (`ADR-035-llm-reasoning.md:246-270`), stating: (a) the real 7 variant names now in code, (b) `PatternMetadata` is the reconciled home for per-pattern configuration, (c) `ReAct`/`PlanExecute`/`SelfConsistency`/`LeastToMost` are retired names with no implementation, and (d) pointing to this ADR (ADR-049) for the real-vs-deferred status of each surviving pattern.

### Per-pattern status after this ADR

| Pattern | ADR-035 analog | Status |
|---|---|---|
| `ChainOfThought` | `CoT` | Honest baseline (`LinearReasoning`) — single path, no claim beyond that; correctly needs no change |
| `Reflection` | `Reflexion` (renamed) | **REAL** — `ReflectiveReasoning` (this ADR) |
| `TreeOfThought` | `TreeOfThought` | **REAL** — `BranchingReasoning` (this ADR) |
| `FewShot` | — (no analog) | Deferred — still served by `LinearReasoning`, explicitly not implied as real |
| `MultiAgent` | — (no analog) | Deferred — same |
| `Hierarchical` | `PlanExecute` (loose analog) | Deferred — same |
| `GraphBased` | — (no analog) | Deferred — same |
| — | `ReAct` | Retired name, no code analog, documented in ADR-035 amendment |
| — | `SelfConsistency` | Retired name, no code analog, documented in ADR-035 amendment |
| — | `LeastToMost` | Retired name, no code analog, documented in ADR-035 amendment |

## What this ADR explicitly does NOT solve

- **`FewShot`/`MultiAgent`/`Hierarchical`/`GraphBased` remain on the honest `LinearReasoning` baseline** — not fixed here, and not to be implied as fixed once `PatternDispatchReasoning` exists. Each is real, separate follow-on work, one pattern at a time (see Tracking).
- **No real vendor `Completer` yet** (ADR-048, still Proposed) — `ReflectiveReasoning`/`BranchingReasoning` will run against whatever `Completer` is registered, today `EchoCompleter`/`NoopCompleter` only. This ADR proves the reasoning-side plumbing (a genuine second call; genuine concurrent branches with real selection), not model quality.
- **No caller-selectable pattern over HTTP.** `DefaultReasoningHandler::Request` stays a bare `String` per ADR-024/ADR-035's connection-contract amendment (`ADR-035-llm-reasoning.md:246-256`) — a wire-level caller still cannot choose a pattern; only in-process/library callers (e.g. `edge-llm-agent`, which already depends on `edge-llm-complete`) get real Reflection/TreeOfThought behavior by calling `Reasoning::reason()` directly. Closing this is a separate, follow-on wire-contract ADR, not a rider here.
- **Branch selection in `BranchingReasoning` is a first cut** (highest `evaluate_step` confidence among 7 parallel branches) — not a tuned or ML-based tree search, no pruning, no backtracking across multiple rounds.
- **No token-budget/cost accounting per pattern.** ADR-035's `cost_multiplier`/`PatternConfig.estimated_tokens` idea is not resurrected; `TokenCounter`/context-window joining remains the pre-existing gap ADR-045 already named.
- **Tool governance (ADR-046) is not applied to the `Completer` calls made inside reasoning.** If a future `ReflectiveReasoning`/`BranchingReasoning` gains tool-calling ability, that composition (likely `GovernedHandler` wrapping, per ADR-046) is separate wiring, not assumed here.

## Consequences

**What this enables**
- Two of seven pattern names finally do something distinct and inspectable: Reflection literally reflects (a real second self-critique completion call), Tree-of-Thought literally branches (real concurrent completions, real selection) — closing the specific fake-work gap ADR-045's audit flagged, for the two highest-value, cheapest-to-implement patterns first.
- `DefaultReasoningHandler`'s latent pattern-binding bug (`execute()` ignoring the constructed pattern) is fixed as a direct byproduct, improving correctness even for the unchanged/deferred patterns.
- Reuses `edge_pipeline::ParallelStepSvc` and `edge_llm_complete::Completer` exactly as already proven in `edge-llm-complete`'s own `BoundedToolCallLoop` — no new concurrency primitive, no new HTTP-completion abstraction invented.
- `PatternDispatchReasoning` gives later ADRs a router to extend one pattern at a time (e.g. `FewShot` next) without re-touching Reflection/TreeOfThought once they're done.
- ADR-035's originally-intended pattern set is finally reconciled with reality via a documented amendment, rather than staying silently stale.

**What this requires**
- New dependency edge `edge-llm-reasoning → edge-llm-complete` (`reasoning/Cargo.toml` gains `edge-llm-complete.workspace = true`) — verified acyclic.
- 4 new `core/reasoning/` files (`reflective_reasoning.rs`, `branching_reasoning.rs`, `reasoning_branch_batch.rs`, `pattern_dispatch_reasoning.rs`) + 1 fixed file (`default_reasoning_handler.rs`).
- Full `_happy`/`_error`/`_edge` test coverage for `ReflectiveReasoning`, `BranchingReasoning`, `PatternDispatchReasoning`, per this repo's test standards, using a small `pub(crate)` stub `Completer` local to reasoning's own tests (not `EchoCompleter`, to avoid a test-only dependency on `complete`'s internals, and to steer clear of the `no_mocks_in_integration` arch-audit naming trap already logged from prior sessions).
- A third amendment to `ADR-035-llm-reasoning.md` — documentation only, no code change required by the amendment itself.
- `arch audit --rs` re-run on `edge-llm-reasoning` after the new files land (currently believed 169/172 per prior-session tradeoff notes; must stay at parity or better, not regress).

## Alternatives Considered

**Make all 7 patterns real in one ADR**
Rejected. Too large for one ADR/PR, and would repeat the exact "declare-and-abandon" risk this ADR is meant to close — five more patterns with no proven-cheap primitive to reuse (unlike Reflection/ToT) would mean inventing new mechanics under time pressure rather than reusing what's proven. Phased, cheap-primitive-first delivery mirrors ADR-045's own "provider registered first because it needs no prerequisite work" discipline.

**Reconcile the `ReasoningPattern` enum to ADR-035's original 7 names/per-variant fields**
Rejected. Would require inventing enum payload fields with zero consumers (recreating the fakeness this ADR fixes, just relocated), and would abandon `PatternMetadata`'s already-working reconciliation of the same "configuration per pattern" need. See Decision.

**Give each pattern its own crate/plugin (ADR-035's "Future Work: Custom patterns — Plugin SDK")**
Rejected for now. With only 2 of 7 patterns real after this ADR, a plugin system is premature — four patterns still correctly share one honest fallback (`LinearReasoning`); a plugin SDK is worth revisiting once more than two patterns have distinct, real implementations to decouple.

**Extend `Handler::Request` to carry `ReasoningPattern` in this same ADR, closing the HTTP-selectability gap**
Rejected as in-scope-here. ADR-024 (referenced by ADR-035's 2026-06-18 amendment) deliberately fixed `Handler<Request = String>` for this crate; changing the wire contract is an orthogonal, separately-reviewable decision and shouldn't ride in on an ADR whose job is fixing pattern *behavior*, not the transport contract.

## Tracking

- Issue: fix `DefaultReasoningHandler`'s hardcoded `REASONING_DEFAULT_PATTERN` bug (store/pass through the bound `pattern` field) — small, independent, should land regardless of the rest of this ADR's disposition
- Issue: implement `ReflectiveReasoning` (`core/reasoning/reflective_reasoning.rs`) + `_happy`/`_error`/`_edge` tests
- Issue: implement `BranchingReasoning` + `DefaultReasoningBranchBatch` (reuse `ParallelStepSvc`) + tests
- Issue: implement `PatternDispatchReasoning` router + wire as the new default reasoner in `DefaultReasoningHandler`
- Issue: third amendment to `ADR-035-llm-reasoning.md` — document the real pattern set, `PatternMetadata` reconciliation, and the retired `ReAct`/`PlanExecute`/`SelfConsistency`/`LeastToMost` names
- Follow-up (separate ADR, out of scope here): make `ReasoningPattern` selectable over HTTP via `DefaultReasoningHandler`'s wire contract
- Follow-up (separate ADR, out of scope here): real behavior for `FewShot`/`MultiAgent`/`Hierarchical`/`GraphBased`, one at a time, once Reflection/TreeOfThought are proven in production
- Depends on (not blocking, tracked separately): ADR-048 (real vendor `Completer`) — until it lands, `ReflectiveReasoning`/`BranchingReasoning` are real plumbing over an echo backend, same scoping caveat ADR-045 already applied to itself
