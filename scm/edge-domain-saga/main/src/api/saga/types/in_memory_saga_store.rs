use std::collections::HashMap;

use crate::api::saga::traits::Saga;

/// Stores [`Saga`] instances in a `HashMap` keyed by `SagaId`.
///
/// Reference implementation for development and testing; state is lost when
/// the process stops.
pub struct InMemorySagaStore<S: Saga> {
    pub(crate) sagas: HashMap<S::SagaId, S>,
}

impl<S: Saga> InMemorySagaStore<S> {
    /// Construct an empty store.
    pub fn new() -> Self {
        Self {
            sagas: HashMap::new(),
        }
    }
}

impl<S: Saga> Default for InMemorySagaStore<S> {
    fn default() -> Self {
        Self::new()
    }
}
