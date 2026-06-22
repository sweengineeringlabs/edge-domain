# ISSUE: edge-llm-prompt Implementation Must Align to ADR-034

**Issue Type:** Defect / Design Debt  
**Severity:** High  
**Status:** Open  
**Created:** 2026-06-22  
**Related Epic:** [edge-domain-prompt Architectural Compliance](epic-edge-domain-prompt-arch-remediation.md)  
**Related ADR:** [ADR-034: LLM Prompt Domain Primitive](../docs/adr/ADR-034-llm-prompt.md)  
**Crate:** `domain/llm/prompt`  

---

## Summary

The `edge-llm-prompt` crate implementation does **not follow** the specifications in ADR-034. Four trait contracts are misnamed or missing, the core layer contains implementations (contradicting the ADR specification), and the API boundary leaks concrete types (violating the ADR's public-surface design).

This issue tracks bringing implementation into compliance with ADR-034 or formally amending the ADR to document intentional deviations.

---

## Misalignments

### 1. Trait Contract Mismatch: `PromptTemplate` → `Prompt`

**ADR Specification (§Decision, line 32):**
```rust
pub trait PromptTemplate: Send + Sync {
    fn render(&self, variables: PromptVariables) -> Result<String, PromptError>;
    fn estimated_tokens(&self, variables: &PromptVariables) -> usize;
    fn description(&self) -> &str;
}
```

**Actual Implementation:**
```rust
// domain/llm/prompt/main/src/api/prompt/traits/prompt.rs
pub trait Prompt: Send + Sync {
    // similar methods
}
```

**Impact:**
- ADR contract naming is not honored
- Documentation/examples referencing `PromptTemplate` are incorrect
- Breaks ADR as source of truth

**Fix Required:** Either
- (Option A) Rename impl to `PromptTemplate` + `PromptVariables` to match ADR, OR
- (Option B) Amend ADR-034 to formally rename `PromptTemplate` → `Prompt` with rationale

---

### 2. Missing Trait: `TemplateRenderer`

**ADR Specification (§Decision, line 130):**
```rust
pub trait TemplateRenderer: Send + Sync {
    fn render(&self, template: &str, variables: &[(&str, &str)]) 
        -> Result<String, PromptError>;
    fn extract_variables(&self, template: &str) -> Vec<String>;
    fn validate_variables(&self, template: &str, variables: &[(&str, &str)]) 
        -> Result<(), PromptError>;
}
```

**Actual Implementation:**
- ❌ `TemplateRenderer` trait does not exist
- Template rendering is delegated to plugins

**Impact:**
- ADR specifies a contract that consumers expect
- No template variable validation at domain level
- Implementations cannot interchange template engines

**Fix Required:**
- Implement `TemplateRenderer` trait in `api/prompt/traits/template_renderer.rs`, OR
- Amend ADR-034 §Limitations to formally defer TemplateRenderer to plugins
- Add `template_renderer_svc.rs` to saf/

---

### 3. Factory Pattern: `PromptFactory` → `PromptBootstrap`

**ADR Specification (implied §Architecture, line 218-219):**
```rust
// Inferred from trait naming convention
pub trait PromptFactory: Send + Sync {
    // factory methods
}
pub struct StdPromptFactory;
```

**Actual Implementation:**
```rust
// domain/llm/prompt/main/src/api/prompt/traits/prompt_bootstrap.rs
pub trait PromptBootstrap {
    fn prompt(...) -> Arc<dyn Prompt>;
    // ... other factory methods
}
pub struct StdPromptFactory;
```

**Context:** Issue #96 refactored all `*Factory` traits → `*Bootstrap` traits workspace-wide after ADR-034 was written (2026-06-16).

**Impact:**
- ADR specification is outdated
- Implementation follows newer pattern (post-#96)
- Documentation mismatch

**Fix Required:**
- Amend ADR-034 §Decision to rename `PromptFactory` → `PromptBootstrap`
- Reference issue #96 (commit c486c8c) as justification
- Update examples and workspace layout section

---

### 4. Core Layer Contains Implementations

**ADR Specification (§Architecture, line 235-236):**
```
├── core/
│   └── (empty - implementations in plugins)
```

**Actual Implementation:**
```
core/prompt/
├── default_prompt.rs           (StaticPrompt impl)
├── map_context_manager.rs      (MapContextManager impl)
├── heuristic_token_counter.rs  (HeuristicTokenCounter impl)
└── prompt_cache.rs             (PromptCache impl)
```

**Context:** This pattern matches provider, reasoning, and agent crates. ADR was theoretical ("plugins only"); reality is "reference impls in core, vendor-specific impls in plugins".

**Impact:**
- ADR's "empty core" design was never intended
- Core layer contains 4+ concrete implementation types
- Misaligns with stated architecture

**Fix Required:**
- Amend ADR-034 §Architecture to document reference implementations in core/
- Clarify: "core/ contains reference impls; vendor-specific impls live in plugins"
- List the 3 standard impls: StaticPrompt, MapContextManager, HeuristicTokenCounter

---

### 5. Handler Connection Pattern ✅ (Compliant)

**ADR Specification (§Amendment, line 305-312):**
```
ADR-024 (functional handler pattern):
- DefaultPromptHandler in core/prompt/
- prompt_handler(prompt: Arc<dyn Prompt>) -> impl Handler factory
- Handler id: "prompt.render"
```

**Actual Implementation:**
```rust
// domain/llm/prompt/main/src/spi/prompt/default_prompt_handler.rs
impl Handler for DefaultPromptHandler { ... }
```

**Status:** ✅ **Compliant** — Amendment correctly implemented

**Note:** Test file `prompt_endpoint_int_test.rs` uses old ADR-037 naming; should be renamed to `prompt_handler_svc_int_test.rs` (consistency with other LLM crates).

---

### 6. API Boundary Violation (Public Surface Purity)

**ADR Specification (§Integration Points & Amendment):**
```
lib.rs should export only:
- Trait contracts (Prompt, ContextManager, TokenCounter)
- Factories (PromptBootstrap, StdPromptFactory)
- Value types (PromptMetadata, RenderContext, PromptError)
```

**Actual Implementation:**
```rust
// domain/llm/prompt/main/src/lib.rs
pub use saf::*;  // ← exports ALL 52 types, including concrete impls
```

**Exported Concrete Types (Should NOT be public):**
- `StaticPrompt`
- `MapContextManager`
- `HeuristicTokenCounter`
- `PromptCache`
- builders: `PromptCacheBuilder`, `PromptMetadataBuilder`, `VariableBuilder`

**Impact:**
- **Arch audit failure #2:** `encapsulation.package_access_violation` (52 offenders)
- Public API leaks implementation details
- Consumers can construct impls directly instead of using factories
- SEA Rule 47 violation

**Fix Required:**
- Replace `pub use saf::*` with explicit export list (see epic-edge-domain-prompt-arch-remediation.md)
- This reaches arch audit 183/183

---

## Required Actions

### Phase 1: Decision — Align or Amend?

For each misalignment, the team must decide:

| Item | Misalignment | Option A: Align Impl to ADR | Option B: Amend ADR |
|------|---|---|---|
| PromptTemplate vs Prompt | Trait name | Rename trait to PromptTemplate | Amend: "Prompt (not PromptTemplate)" |
| TemplateRenderer | Missing trait | Implement trait + svc | Amend §Limitations: "deferred to plugins" |
| PromptFactory vs PromptBootstrap | Factory pattern | Rename to PromptFactory | Amend: reference issue #96 for BootstrapPattern |
| Core layer empty | Contains impls | Move impls to plugins | Amend §Architecture: document reference impls |
| Handler pattern | (Already compliant) | N/A | ✅ Keep current amendment |
| API purity | Concrete types exported | Fix lib.rs exports (epic task #2) | N/A |

**Recommended approach:**
- **Option B (Amend ADR)** for items 1-4: Implementation patterns match the workspace standard (provider/reasoning). Codify the pattern.
- **Fix impl** for item 6 (API purity): This is a real arch violation that violates published SEA rules.

### Phase 2: Amend ADR-034

Create amendment dated 2026-06-22 addressing:
1. Trait naming: `PromptTemplate` → `Prompt`
2. Factory pattern: `PromptFactory` → `PromptBootstrap` (link to #96)
3. Core layer: Document reference implementations (StaticPrompt, MapContextManager, HeuristicTokenCounter)
4. TemplateRenderer: Defer to plugins per §Limitations
5. Reaffirm ADR-024 handler pattern amendment (2026-06-18) is correct

### Phase 3: Fix Implementation API Boundary

Complete epic task [edge-domain-prompt-arch-remediation.md](epic-edge-domain-prompt-arch-remediation.md):
- Replace `pub use saf::*` with explicit export list
- Reach arch audit 183/183
- Commit with reference to this issue

### Phase 4: Housekeeping

- Rename test file: `prompt_endpoint_int_test.rs` → `prompt_handler_svc_int_test.rs`
- Update examples/ to reference correct trait names
- Link this issue in epic and arch remediation docs

---

## Acceptance Criteria

- [ ] ADR-034 amendment drafted (Phase 2)
- [ ] ADR amendment merged into main
- [ ] Implementation API boundary fixed (arch 183/183)
- [ ] All implementation examples updated to new trait names
- [ ] Test files renamed for consistency
- [ ] This issue closed with reference to amendment commit + arch fix commit

---

## Related Issues

- **Epic:** [edge-domain-prompt Architectural Compliance — 181/183 → 183/183](epic-edge-domain-prompt-arch-remediation.md)
- **Issue #96:** [refactor(arch): rename *Factory→*Bootstrap across all 25 workspace members](https://github.com/sweengineeringlabs/edge-domain/commit/c486c8c) (c486c8c)
- **Arch rule:** SEA Rule 47 — concrete types must not cross crate boundary

---

## References

- ADR-034: [LLM Prompt Domain Primitive](../../edge/docs/3-architecture/adr/ADR-034-llm-prompt.md)
- ADR-024: [Handler-Execute Contract](../../edge/docs/3-architecture/adr/ADR-024-handler-execute-contract.md)
- ADR-033: [LLM Provider Domain Primitive](../../edge/docs/3-architecture/adr/ADR-033-llm-provider.md) (reference alignment)
- ADR-035: [LLM Reasoning Domain Primitive](../../edge/docs/3-architecture/adr/ADR-035-llm-reasoning.md) (reference alignment)
