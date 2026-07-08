# ADR-055: `edge-llm-eval` — LLM Output Quality Eval Harness

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-033 (LLM Provider), ADR-030/032 (Agent/LLM Agent), ADR-045 (`edge-llm-runtime` Composition Root), ADR-046 (`edge-llm-tools` Governance)
**GitHub Issues:** TBD

---

## Context

This repo's own engineering mandate (`CLAUDE.md`) is fanatical about test quality: "a test that cannot fail is not a test," happy/error/edge scenarios required for every public function, `arch audit --rs` gating every merge at 183/183. Every LLM domain-primitive crate built under this discipline — `edge-llm-provider`, `edge-llm-prompt`, `edge-llm-reasoning`, `edge-llm-complete`, `edge-llm-agent` — has this rigor applied to it. `Provider` (`domain/scm/domain/llm/provider/main/src/api/provider/traits/provider.rs:20-78`) and `Agent` (`domain/scm/domain/llm/agents/main/src/api/traits/agent.rs:17-108`) both have full test coverage per the LLM five-crate mandate (ADR series, all closed at 214/216).

But every one of those tests asserts something about *code* — does `Provider::complete` return the right `Result` variant when the completer errors, does `Agent::execute_skill` route to the right `Skill`. None of them, and nothing else anywhere in the repo, asserts anything about *output quality* — whether a given prompt, model, or `ReasoningPattern` actually produces a good answer. A grep across `domain/` and `edge/` for `EvalCase|EvalRunner|Scorer|benchmark_suite|llm_eval|EvalHarness` (case-insensitive) returns **zero hits**. This gap was already named explicitly in ADR-045's "What this ADR explicitly does NOT solve" (`ADR-045-edge-llm-runtime-standalone-composition-root.md:69`): "No RAG/embeddings/vector retrieval, guardrails/content moderation, cost/usage tracking, **eval harness**, or real multimodal input path anywhere in `edge/`." This ADR closes that specific item.

The gap matters structurally, not just as a missing nice-to-have: `edge-llm-reasoning`'s "multi-strategy reasoning" (flagged in ADR-045 as "one linear loop cloned N times, not distinct pattern behavior") and `edge-llm-agent`'s skill routing both make implicit quality claims — that one `ReasoningPattern` or prompt variant produces better completions than another — that nothing in the repo can currently confirm or refute. Code-correctness tests would pass unchanged whether or not those claims are true.

Two existing ports are directly reusable rather than reinvented:

- `Provider::complete(req: ProviderCompleteRequest) -> Result<ProviderCompletionResponse, ExecutionError>` (`provider.rs:74-77`) — `ProviderCompleteRequest` wraps a `Box<CompletionInput>` (`provider_complete_request.rs:7`), `ProviderCompletionResponse` wraps a `CompletionResponse` with `content: Option<String>` (`provider_completion_response.rs:7`, `completion_response.rs:7-17`). This is the seam an eval harness drives to get an actual model output for a given input.
- `Agent::execute_skill(req: SkillExecutionRequest<'_>) -> Result<SkillExecutionResponse, AgentError>` (`agent.rs:33-36`) — `SkillExecutionRequest` carries `skill_name: &str`, `input: String`, `ctx: HandlerContext<'a>` (`skill_execution_request.rs:4-11`); `SkillExecutionResponse` carries `output: String` (`skill_execution_response.rs:3-6`). This is the seam for evaluating an agent's skill output, not just a raw completion.

An eval harness needs to run cases *against* these two ports, scoring the resulting `content`/`output` strings — it must not reimplement how to call an LLM.

## Decision

Build `edge-llm-eval`, a small domain-primitive crate whose only job is: given an `EvalCase` (input + expected-output-or-rubric) and something that can produce a `Provider`/`Agent` output, produce a `ScoreResponse`, and aggregate many of those into a report. It calls `Provider`/`Agent` as an ordinary consumer of those existing traits; it defines no new way of invoking a model.

### Kept: three new concepts, no existing analog

- **`EvalCase`** — one test input plus a grading criterion. Two variants of criterion, since "expected output" is sometimes an exact string and sometimes a rubric an LLM-judge grades against:
  ```rust
  pub struct EvalCase {
      pub id: String,
      pub input: EvalCaseInput,
      pub criterion: EvalCriterion,
  }

  pub enum EvalCaseInput {
      Completion(CompletionInput),         // reuses edge_llm_provider::CompletionInput
      Skill { skill_name: String, input: String }, // reuses Agent::execute_skill's shape
  }

  pub enum EvalCriterion {
      ExactMatch { expected: String },
      Rubric { description: String },      // graded by an LLM-judge Scorer, not string-compared
  }
  ```
  `EvalCase` deliberately does not define its own "how do I call the model" logic — `EvalCaseInput` is a thin enum over the two existing request shapes (`CompletionInput` from `edge-llm-provider`, and the `skill_name`/`input` pair `Agent::execute_skill` already takes).

- **`Scorer` trait** — scores one actual output against one `EvalCase`. Request/Response shaped per the mandatory port pattern:
  ```rust
  pub trait Scorer: Send + Sync {
      fn score(&self, req: ScoreRequest<'_>) -> Result<ScoreResponse, EvalError>;
  }

  pub struct ScoreRequest<'a> {
      pub case: &'a EvalCase,
      pub actual_output: &'a str,
  }

  pub struct ScoreResponse {
      pub passed: bool,
      pub score: f32,           // 0.0..=1.0, so partial-credit scorers (LLM-judge) aren't forced into a bool
      pub explanation: String,  // required, not optional — "why" is not free to omit; a bare pass/fail is not actionable
  }
  ```
  Two implementations ship in `core/`:
  - `ExactMatchScorer` — `passed = actual_output.trim() == expected.trim()`, `score` is `1.0`/`0.0`. Only valid against `EvalCriterion::ExactMatch`; returns `EvalError::CriterionMismatch` against a `Rubric` case.
  - `LlmJudgeScorer` — holds an `Arc<dyn Provider>`, builds a grading `CompletionInput` from the `Rubric` description plus the actual output, calls `Provider::complete`, and parses the judge's response into a `passed`/`score`/`explanation`. This is the one place the crate's own port composes with itself — a `Scorer` that is also a `Provider` consumer — and is why `LlmJudgeScorer` lives in `core/` next to `ExactMatchScorer` rather than being hand-waved as "bring your own": it's proof the contract holds together, not just a paper design. Malformed/unparseable judge output surfaces as `EvalError::JudgeResponseMalformed`, not a panic or a silent `passed: false`.

- **`EvalRunner` trait** — runs a set of `EvalCase`s against a `Provider` or `Agent` and aggregates `Scorer` results:
  ```rust
  pub trait EvalRunner: Send + Sync {
      fn run(&self, req: EvalRunRequest<'_>) -> Result<EvalRunResponse, EvalError>;
  }

  pub struct EvalRunRequest<'a> {
      pub cases: &'a [EvalCase],
      pub scorer: &'a dyn Scorer,
  }

  pub struct EvalRunResponse {
      pub results: Vec<EvalCaseResult>,   // one per case, in input order — no silent drops on partial failure
      pub pass_rate: f32,                 // results.iter().filter(|r| r.passed).count() as f32 / results.len() as f32
  }

  pub struct EvalCaseResult {
      pub case_id: String,
      pub score: ScoreResponse,
      pub call_error: Option<String>,     // Provider/Agent call itself failed — distinct from a low score
  }
  ```
  `EvalRunner::run` is deliberately *not* generic over `Provider`/`Agent` at the trait level — a `Default*EvalRunner` in `core/` is constructed with an `Arc<dyn Provider>` (for `EvalCaseInput::Completion`) or `Arc<dyn Agent>` (for `EvalCaseInput::Skill`), matching each case's input variant to the right port and recording a per-case `call_error` (not an aborted run) when the port itself errors — one bad case must not silently drop the rest of the report.

- **`EvalError`** — the one new error type, satisfying `api_error_type_named`:
  ```rust
  pub enum EvalError {
      CriterionMismatch { scorer: &'static str, criterion: &'static str },
      JudgeResponseMalformed(String),
      ProviderCallFailed(String),   // wraps ExecutionError::to_string(), not the type itself — Scorer/EvalRunner don't leak provider-specific error shapes
      AgentCallFailed(String),      // wraps AgentError::to_string()
      EmptyCaseSet,
  }
  ```

### Shape / workspace layout

```
domain/scm/domain/llm/eval/            (edge-llm-eval)
├── api/
│   ├── traits/{scorer.rs, eval_runner.rs}
│   ├── types/{eval_case, eval_case_input, eval_criterion,
│   │           score_request, score_response,
│   │           eval_run_request, eval_run_response, eval_case_result}.rs
│   └── errors/eval_error.rs
├── core/
│   ├── exact_match_scorer.rs        (impl Scorer, ExactMatch only)
│   ├── llm_judge_scorer.rs          (impl Scorer, holds Arc<dyn Provider>)
│   └── default_eval_runner.rs       (impl EvalRunner, holds Arc<dyn Provider> + Option<Arc<dyn Agent>>)
└── saf/
    ├── scorer_svc.rs                 (pub use api::traits::Scorer)
    └── eval_runner_svc.rs            (pub use api::traits::EvalRunner)
```

Depends on: `edge-llm-provider` (`Provider`, `CompletionInput`, `ExecutionError`), `edge-llm-agent` (`Agent`, `SkillExecutionRequest/Response`, `AgentError`). No dependency on `edge-domain-handler`, `edge-domain-observer`, or `edge-security-runtime` — see below.

### Does this need `Handler`/`HandlerContext` wiring?

**No.** This is deliberately modeled as a standalone dev-tool contract, not an ingress-facing port, for three concrete reasons:

1. **Nothing external calls it at request-serving time.** Every other primitive in this ADR series (`edge-llm-runtime`'s `Default*Handler`s, ADR-046's `GovernedHandler`) exists because an HTTP/gRPC request needs to reach domain logic through `edge-dispatcher`'s registry. An eval run has no such caller — it's invoked by a developer or a CI job, not by production traffic. Giving it a `Handler` impl would mean threading a `HandlerContext` (security principal, observer span) through a code path that never serves a live request, for no consumer that needs it.
2. **`Agent::execute_skill` already requires a `HandlerContext` internally** (`SkillExecutionRequest<'a>.ctx: HandlerContext<'a>`, `skill_execution_request.rs:9-10`). `EvalRunner` satisfies that requirement locally when it builds the `SkillExecutionRequest` for an `EvalCaseInput::Skill` case — it constructs a minimal, real `HandlerContext` (with `Noop` observer/security where the harness itself has no principal to report) at the call site. That's an internal implementation detail of driving `Agent`, not a reason for `EvalRunner`/`Scorer` to *themselves* be `Handler`s.
3. **Precedent already exists for standalone-tool shape in this domain.** `edge-llm-runtime` (ADR-045) drew a hard line between "domain primitive" (library, no transport) and "composition root" (application, has transport). `edge-llm-eval` is not even a composition root — it's closer to a test harness invoked by a CLI or a `cargo test`/CI step, the same way `arch audit --rs` itself is a standalone tool that reads a workspace and produces a report, not a `Handler` registered anywhere. The natural consumer is a thin CLI binary (out of scope for this ADR — see below) that loads `EvalCase`s from a file, constructs a `Default*EvalRunner`, and prints the `EvalRunResponse`.

If a future need arises to run evals *as* a registered, HTTP-triggered job (e.g. "kick off a nightly eval via API call"), that's an ingress concern layered on top — wrap `EvalRunner::run` in a `Handler` impl in a composition root, the same pattern ADR-046 used for `GovernedHandler`. Nothing about `EvalRunner`'s own trait shape blocks that later; nothing about it requires it now.

## What this ADR explicitly does NOT solve

- **Not a regression-CI gate.** Running `edge-llm-eval` in CI, defining pass/fail thresholds ("fail the build if `pass_rate < 0.9`"), and tracking pass-rate trends over time/across commits are composition-root and tooling concerns, layered on top of this contract — not part of it. `EvalRunResponse` reports a `pass_rate` for a single run; deciding what to *do* with that number belongs to whatever CI step or dashboard consumes it, not to this crate.
- **No golden-dataset management.** Where `EvalCase`s live, how they're versioned, curated, or reviewed is out of scope — `EvalCase` is a plain data type; loading a `Vec<EvalCase>` from a file/fixture/database is a caller concern.
- **No statistical rigor beyond a raw pass rate.** No confidence intervals, no significance testing between two runs, no A/B comparison between model/prompt variants. `EvalRunResponse` is a single flat report, not a comparison framework.
- **No cost/latency tracking during eval runs.** `Provider::last_token_usage` (`provider.rs:43-46`) already exists and could be read by a caller wrapping `EvalRunner`, but this crate doesn't aggregate it.
- **No streaming-output evaluation.** Only whole `content: Option<String>` / `output: String` results are scored, matching `Provider::complete`'s and `Agent::execute_skill`'s current non-streaming shapes.
- **`LlmJudgeScorer`'s own judge quality is not validated by this ADR.** Using an LLM to grade an LLM is a known-imperfect technique (judge bias, inconsistency across runs); this ADR provides the composable seam for it, not a claim that judge grades are ground truth.

## Consequences

**What this enables**
- A real answer, for the first time, to "does this prompt/model/reasoning-pattern actually produce good output" — closing the exact gap named in ADR-045's not-solved list.
- `edge-llm-reasoning`'s multiple `ReasoningPattern`s and `edge-llm-prompt`'s variants can be compared empirically (run the same `EvalCase` set through each, compare `pass_rate`) instead of only being code-tested for "does it run without erroring."
- A concrete, minimal validation that `Provider` composes with itself: `LlmJudgeScorer` is a `Scorer` that internally calls `Provider::complete` to grade another `Provider::complete` call's output.
- A reusable seam for both raw completions (`Provider`) and full agent/skill runs (`Agent`) under one `EvalCase`/`Scorer`/`EvalRunner` shape, rather than two disconnected ad hoc scripts.

**What this requires**
- New crate `edge-llm-eval` under `domain/scm/domain/llm/eval/`.
- No changes to `Provider`, `Agent`, `edge-domain-handler`, or any existing LLM crate's `api/`.
- A consumer to actually load `EvalCase` fixtures and invoke `EvalRunner` — this ADR ships the contract, not a curated eval dataset or a CLI binary; a small CLI (or a `cargo xtask`-style entry point) that wires `Default*EvalRunner` to a `Provider`/`Agent` instance and prints `EvalRunResponse` is natural, cheap follow-up work, not bundled here to keep this ADR's contract minimal.

## Alternatives Considered

**Bolt eval scoring onto `edge-llm-runtime` as a registered `Handler`**
Rejected. There is no request-serving reason for an eval run to be reachable over HTTP/gRPC in the first place — its caller is a developer or CI job, not live traffic. Forcing a `HandlerContext` through it would mean fabricating a security principal and observer span for a code path that structurally has neither, purely to fit a shape (ADR-045's ingress chain) built for a different problem.

**Make `Scorer`/`EvalRunner` generic over any callable (`Fn(String) -> String`) instead of the concrete `Provider`/`Agent` traits**
Rejected. This repo already has two purpose-built, arch-audited ports for "get a completion" (`Provider::complete`) and "get a skill result" (`Agent::execute_skill`); reinventing a third, weaker abstraction over them would duplicate what those traits already guarantee (structured errors, model metadata, token usage) for no benefit, and would violate the same "compose existing ports, don't reinvent calling an LLM" principle this ADR is built on.

**Skip the `ExactMatch` variant and require every `EvalCase` to use an `LlmJudgeScorer`**
Rejected. Many eval cases have a genuinely deterministic expected string (e.g. "does this prompt produce valid JSON with field `x: 1`") where an exact/structural comparison is strictly more reliable and far cheaper than an LLM-judge round trip. Forcing every case through a judge would also make the crate's own tests non-deterministic (an `ExactMatchScorer`'s tests need no live model at all), undermining "a test that cannot fail is not a test" for the harness's own test suite.

**Build this as a standalone script/notebook outside the arch-audited domain crates**
Rejected. `EvalCase`/`Scorer`/`EvalRunner` are exactly the kind of stable contract other crates and future composition roots need to depend on (e.g. a CI tool, a future `edge-llm-eval-runtime`) — leaving it as an unaudited script would mean no `api_error_type_named` discipline on `EvalError`, no test-scenario coverage requirement, and no reusable trait for a second consumer to build against.

## Tracking

- New crate: `edge-llm-eval` (`domain/scm/domain/llm/eval/`)
- Follow-up (not blocking this ADR): a thin CLI/`xtask` entry point that loads `EvalCase` fixtures and invokes `Default*EvalRunner` — the actual first consumer
- Follow-up (explicitly out of scope, see "does NOT solve"): CI gate wiring, pass-rate thresholds, historical trend tracking
- Depends on no changes to `edge-llm-provider` or `edge-llm-agent` — pure downstream consumer of both
