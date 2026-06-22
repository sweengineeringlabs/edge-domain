# ISSUE #: Align edge-llm-prompt Implementation to ADR-034

**Epic:** [edge-domain-prompt Architectural Compliance — 181/183 → 183/183](epic-edge-domain-prompt-arch-remediation.md)  
**Issue Document:** [issue-edge-domain-prompt-adr-alignment.md](issue-edge-domain-prompt-adr-alignment.md)  
**Crate:** `domain/llm/prompt`  
**Status:** Open  
**Created:** 2026-06-22 (commit 56c09c1)  

---

## Problem Statement

Implementation of `edge-llm-prompt` does not follow ADR-034 specifications. Six trait contracts, factory patterns, and API boundaries diverge from the published architecture decision record without formal amendments.

**Link to detailed analysis:** [issue-edge-domain-prompt-adr-alignment.md](issue-edge-domain-prompt-adr-alignment.md)

---

## Tasks (Subtasks of Epic)

### Task Group 1: Decision — Align or Amend?

- [ ] **Task 1.1** — Review misalignments against workspace patterns (provider, reasoning, agent crates)
  - Compare ADR-034 PromptTemplate pattern vs. ADR-033 Provider pattern
  - Document which deviations are intentional/justified
  - Assessment: ~1 hour

- [ ] **Task 1.2** — Decide for each misalignment: Align impl or amend ADR?
  - PromptTemplate vs Prompt → Decide
  - TemplateRenderer missing → Decide
  - PromptFactory vs PromptBootstrap → Decide (likely amend; reference #96)
  - Core layer empty vs populated → Decide (likely amend; matches pattern)
  - Assessment: ~30 min

### Task Group 2: Amend ADR-034

- [ ] **Task 2.1** — Draft ADR-034 amendment (2026-06-22)
  - Rename PromptTemplate → Prompt (with rationale)
  - Rename PromptFactory → PromptBootstrap (link to #96/c486c8c)
  - Document core/ reference implementations (StaticPrompt, MapContextManager, HeuristicTokenCounter)
  - Defer TemplateRenderer to plugins §Limitations
  - Reaffirm ADR-024 handler pattern (DefaultPromptHandler)
  - Effort: ~1 hour

- [ ] **Task 2.2** — Merge amendment into ADR-034
  - Create amendment dated 2026-06-22
  - Commit with reference to issue and epic
  - Effort: ~15 min

### Task Group 3: Fix Implementation API Boundary

- [ ] **Task 3.1** — Replace `pub use saf::*` with explicit exports
  - *Linked to epic task:* [#2 - Resolve encapsulation.package_access_violation](epic-edge-domain-prompt-arch-remediation.md)
  - Export only: traits (Prompt, ContextManager, TokenCounter), factories (PromptBootstrap, StdPromptFactory), value types (PromptMetadata, RenderContext, PromptError)
  - Remove concrete impl exports (StaticPrompt, MapContextManager, HeuristicTokenCounter, PromptCache)
  - Effort: ~1 hour

- [ ] **Task 3.2** — Verify arch audit 183/183
  - Run `arch audit --rs` in prompt crate
  - Confirm all 183 checks pass
  - Confirm zero clippy warnings
  - Effort: ~15 min

### Task Group 4: Housekeeping & Consistency

- [ ] **Task 4.1** — Rename test files for consistency
  - Rename: `prompt_endpoint_int_test.rs` → `prompt_handler_svc_int_test.rs`
  - Aligns with provider/reasoning/agent test naming convention
  - Effort: ~15 min

- [ ] **Task 4.2** — Update examples/ to use correct trait names
  - Update examples/prompt.rs to reference `Prompt` (not `PromptTemplate` from ADR)
  - Update factory examples to use `PromptBootstrap` (not `PromptFactory`)
  - Effort: ~15 min

- [ ] **Task 4.3** — Update README.md
  - Reference corrected trait names
  - Link to amended ADR-034
  - Effort: ~10 min

### Task Group 5: Integration & Closure

- [ ] **Task 5.1** — Link ADR amendment commit to this issue
  - Record amendment commit SHA
  - Record arch fix commit SHA
  - Effort: ~5 min

- [ ] **Task 5.2** — Close issue with summary
  - Link all related commits
  - Document decisions made
  - Effort: ~5 min

---

## Related Tasks in Epic

This issue is **part of** [edge-domain-prompt Architectural Compliance Epic](epic-edge-domain-prompt-arch-remediation.md):

- Epic Task #1: Fix prompt root_whitelist violation (cleanup)
- Epic Task #2: Resolve encapsulation.package_access_violation ← **Linked to this issue Task 3.1**
- Epic Task #3: Retrofit completed issues #88-#95 (documentation)
- Epic Task #4: Verify and commit prompt arch remediation

---

## Acceptance Criteria

**All tasks complete when:**
- [ ] ADR-034 amendment merged (Tasks 2.1, 2.2)
- [ ] lib.rs exports fixed to reach arch 183/183 (Tasks 3.1, 3.2)
- [ ] Test/example files renamed for consistency (Tasks 4.1, 4.2, 4.3)
- [ ] Issue closed with link to amendment + arch fix commits (Tasks 5.1, 5.2)
- [ ] This issue linked in epic

---

## Effort Summary

| Task Group | Tasks | Est. Time |
|---|---|---|
| 1. Decision | 2 | 1.5 hours |
| 2. Amend ADR | 2 | 1.25 hours |
| 3. Fix API | 2 | 1.25 hours |
| 4. Housekeeping | 3 | 40 min |
| 5. Integration | 2 | 10 min |
| **Total** | **11** | **~5 hours** |

---

## Links

- **Detailed Issue Analysis:** [issue-edge-domain-prompt-adr-alignment.md](issue-edge-domain-prompt-adr-alignment.md)
- **Epic:** [edge-domain-prompt Architectural Compliance](epic-edge-domain-prompt-arch-remediation.md)
- **ADR-034:** [LLM Prompt Domain Primitive](../../edge/docs/3-architecture/adr/ADR-034-llm-prompt.md)
- **Related Refactor:** Issue #96 (commit c486c8c) — rename *Factory→*Bootstrap
- **Arch Audit Failure:** encapsulation.package_access_violation (52 offenders)

---

## Related Documentation

- [ADR-033: LLM Provider](../../edge/docs/3-architecture/adr/ADR-033-llm-provider.md) — reference implementation pattern
- [ADR-035: LLM Reasoning](../../edge/docs/3-architecture/adr/ADR-035-llm-reasoning.md) — reference implementation pattern
- [ADR-024: Handler-Execute Contract](../../edge/docs/3-architecture/adr/ADR-024-handler-execute-contract.md) — handler pattern (compliant)
