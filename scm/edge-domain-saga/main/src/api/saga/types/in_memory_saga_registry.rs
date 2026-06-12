use std::collections::HashMap;

use crate::api::saga::traits::Saga;

/// Stores [`Saga`] instances in a `HashMap` keyed by `SagaId`.
///
/// Reference implementation for development and testing; state is lost when
/// the process stops.
pub struct InMemorySagaRegistry<S: Saga> {
    pub(crate) sagas: HashMap<S::SagaId, S>,
}

impl<S: Saga> InMemorySagaRegistry<S> {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            sagas: HashMap::new(),
        }
    }
}

impl<S: Saga> Default for InMemorySagaRegistry<S> {
    fn default() -> Self {
        Self::new()
    }
}
