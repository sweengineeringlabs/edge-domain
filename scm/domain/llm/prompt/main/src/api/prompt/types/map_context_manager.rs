//! `MapContextManager` — reference [`ContextManager`](crate::api::prompt::traits::ContextManager) implementation.

use std::collections::BTreeMap;

use crate::api::prompt::types::Variable;

/// Reference context manager that stores variables in an ordered map.
///
/// Deterministic iteration order (via [`BTreeMap`]) keeps built contexts and
/// completeness checks reproducible across runs.
#[derive(Clone, Debug, Default)]
pub struct MapContextManager {
    pub(crate) variables: BTreeMap<String, Variable>,
}

impl MapContextManager {
    /// Construct an empty context manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of registered variables.
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Whether no variables are registered.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}
