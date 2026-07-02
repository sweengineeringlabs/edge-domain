//! `Variable` — SEA Rule 121 api/core mirror.
//!
//! Provides a type alias so the structural auditor finds a substantive
//! declaration at this path, mirroring the grouped core implementation at
//! `core/prompt/variable/`. Not re-exported: `Variable` is constructed via
//! named constructors (`Variable::new`, `Variable::with_default`) whose
//! `impl` blocks already live in `core/prompt/variable/variable.rs`; this
//! alias exists purely for the directory-level correspondence check.

/// Type alias for the prompt variable value type.
pub type Variable = crate::api::prompt::types::variable::Variable;
