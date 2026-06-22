# EPIC: edge-domain-prompt Architectural Compliance — 181/183 → 183/183

**Status:** Open  
**Created:** 2026-06-22 (commit 5103735)  
**Epic Owner:** Senior Agentic Engine Engineer  
**GitHub Epic:** [#97 - Align edge-llm-prompt implementation to ADR-034](https://github.com/sweengineeringlabs/edge-domain/issues/97)  

## Related ADR
**ADR-034:** [LLM Prompt Domain Primitive](../docs/adr/ADR-034-llm-prompt.md)  
**Status:** Accepted (2026-06-16), Amended (2026-06-18 — ADR-024 handler pattern)  
**Needs Amendment:** Yes — 6 misalignments between spec and implementation (issue #97)  
**Crate:** `domain/llm/prompt` (edge-llm-prompt v0.1.0)  

---

## Summary

Resolve architectural audit failures in the `edge-llm-prompt` crate to reach full compliance (183/183). Currently passing 181/183 with 2 failures. This epic tracks remediation of both the trivial cleanup failure and the significant encapsulation violation affecting public API surface.

---

## Current State

**Arch Audit Score:** 181/183 (2 failures)

### Failure 1: `root_whitelist` (Trivial)
- **Severity:** Low (cosmetic)
- **Description:** Unexpected entries at feature level (`.claude`, `arch_audit.txt`)
- **Cause:** Temporary files from audit run
- **Fix:** Delete `.claude/` directory and `arch_audit.txt` file from repo root

### Failure 2: `encapsulation.package_access_violation` (Significant)
- **Severity:** High (public API purity)
- **Description:** lib.rs re-exports 52 concrete implementation types from saf/ onto public crate surface
- **Affected Types:** HeuristicTokenCounter, MapContextManager, PromptCache, StaticPrompt, PromptMetadata, RenderContext, etc.
- **Cause:** lib.rs uses `pub use saf::*` exporting concrete impls; only trait contracts should cross crate boundary
- **Impact:** Violates SEA Rule 47 — concrete implementation types leak into public API

---

## Root Cause Analysis

The prompt crate follows SEA layering (api/ → core/ → saf/) but **violates the public-surface encapsulation rule**:

```rust
// lib.rs (CURRENT — WRONG)
pub use saf::*;  // ← re-exports ALL 52 types including concrete impls
```

**Should be:**

```rust
// lib.rs (CORRECT)
pub use saf::{
    PromptFactory, StdPromptFactory,  // trait contracts + standard factory
    // (NO concrete types)
};
```

### What lib.rs should export:
1. **Traits** (api/ contracts):
   - `Prompt`
   - `ContextManager`
   - `TokenCounter`

2. **Factories** (saf/ facade):
   - `PromptFactory` (trait)
   - `StdPromptFactory` (concrete, but factory pattern is allowed)

3. **Value Types** (api/ vocabulary):
   - `PromptMetadata`
   - `RenderContext`
   - `PromptError`
   - Other value types that belong to contracts

### What lib.rs should NOT export:
- `StaticPrompt` (concrete impl of Prompt — belongs in core/)
- `MapContextManager` (concrete impl of ContextManager — belongs in core/)
- `HeuristicTokenCounter` (concrete impl of TokenCounter — belongs in core/)
- `PromptCache` (concrete impl detail)
- Any other `core/` implementations

---

## Remediation Plan

### Phase 1: Identify Exportable Types
- [ ] Audit all 52 concrete types currently exported
- [ ] Classify each as: (a) trait contract, (b) factory, (c) value type, (d) implementation detail
- [ ] Document which types are safe to export (belong in api/ or saf/ contracts)

### Phase 2: Refactor lib.rs
- [ ] Replace `pub use saf::*` with explicit list of allowed exports
- [ ] Exports: trait contracts from api/, factories from saf/, vocabulary types from api/
- [ ] Validate: only trait names + factory names + value types, zero concrete impls

### Phase 3: Verify Consumers
- [ ] Check if any internal tests/examples depend on private types being public
- [ ] Update test fixtures to use factories + traits instead of direct construction
- [ ] Ensure no external test breakage

### Phase 4: Verify Arch Audit
- [ ] Run `arch audit --rs` in prompt crate
- [ ] Confirm 183/183 pass
- [ ] Confirm zero clippy warnings: `cargo clippy -- -D warnings`
- [ ] Confirm tests pass: `cargo test`

### Phase 5: Document & Commit
- [ ] Update prompt/README.md if API surface changed
- [ ] Commit with message: `fix(llm-prompt): resolve encapsulation violations, reach 183/183 arch audit`
- [ ] Reference this epic in commit body

---

## Related Work

### Similar Arch Fixes (Reference)
- **Provider crate:** Issues #77, #78 — reached 183/183 in commit 12194fb
- **Complete crate:** Issues #79-84 (known limitations tracked) — reached 183/183 in commit f357770
- **Pattern:** Concrete implementation types must never be part of public crate surface

### Blocked By
None — this is independent work.

### Blocks
None — public API changes are backward-incompatible but the crate is early-stage (v0.1.0).

---

## Blocking Issue

### ISSUE #97: ADR Alignment (MUST COMPLETE FIRST)
**GitHub:** [#97 - Align edge-llm-prompt implementation to ADR-034](https://github.com/sweengineeringlabs/edge-domain/issues/97)  
**Relates to ADR:** [ADR-034: LLM Prompt Domain Primitive](../docs/adr/ADR-034-llm-prompt.md)  
**Documentation:** [issue-edge-domain-prompt-adr-alignment-tasks.md](issue-edge-domain-prompt-adr-alignment-tasks.md)  
**Status:** Open (Blocking this epic)

Before implementing arch remediation, implementation must align to ADR-034:
- 6 misalignments documented
- 11 subtasks across 5 task groups
- Estimated effort: ~5 hours
- Decisions required: Amend ADR or align implementation
- **Blocks all tasks below until closed**

## Epic Tasks (Linked to Issue #97)

### Task #1: Fix prompt root_whitelist violation
**Links:** [Epic](epic-edge-domain-prompt-arch-remediation.md) | [Issue #97](#blocking-issue)  
**Description:** `fix(prompt): remove concrete types from lib.rs public surface (root_whitelist cleanup)`  
**Work:** Delete .claude and arch_audit.txt from repo root  
**Effort:** 15 min  
**Arch Impact:** Reaches 182/183

### Task #2: Resolve encapsulation.package_access_violation  
**Links:** [Epic](epic-edge-domain-prompt-arch-remediation.md) | [Issue #97](#blocking-issue)  
**Description:** `fix(prompt): resolve encapsulation.package_access_violation — concrete impls leaking to public API`  
**Work:** Replace `pub use saf::*` with explicit exports (only traits, factories, value types)  
**Effort:** 1 hour  
**Arch Impact:** Reaches 183/183 (completes epic)

### Task #3: Retrofit completed issues #88-#95
**Links:** [Epic](epic-edge-domain-prompt-arch-remediation.md) | [Issue #97](#blocking-issue)  
**Description:** `docs: retrofit completed issues #88-#95 with epic/ADR documentation links`  
**Work:** Create tracking entries linking each issue to its ADR/epic  
**Effort:** 1 hour

### Task #4: Verify and commit prompt arch remediation
**Links:** [Epic](epic-edge-domain-prompt-arch-remediation.md) | [Issue #97](#blocking-issue)  
**Description:** `verify: test + audit + push prompt arch compliance to 183/183`  
**Work:** Run tests, arch audit, clippy, commit  
**Effort:** 30 min

---

## Effort Estimate

- **Phase 1 (Audit):** 30 min — review all exports, classify each type
- **Phase 2 (Refactor):** 1 hour — edit lib.rs, test locally
- **Phase 3 (Verify Consumers):** 30 min — scan tests, update fixtures
- **Phase 4 (Verify Audit):** 15 min — run tools, confirm pass
- **Phase 5 (Commit):** 15 min — message + docs

**Total:** ~3 hours

---

## Definition of Done

- [ ] Arch audit: 183/183
- [ ] Tests: all pass (`cargo test`)
- [ ] Lint: zero warnings (`cargo clippy -- -D warnings`)
- [ ] Format: compliant (`cargo fmt --check`)
- [ ] Commit message references this epic
- [ ] README updated (if API changed)
- [ ] Sub-issues closed

---

## References

- ADR-034: [LLM Prompt Domain Primitive](../../edge/docs/3-architecture/adr/ADR-034-llm-prompt.md)
- SEA Rule 47: Concrete types must not cross crate boundary
- Provider remediation: commit 12194fb (edge-domain#77, #78)
- Complete remediation: commit f357770 (edge-domain#79-84, edge#265)
