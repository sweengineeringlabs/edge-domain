use crate::api::saga::traits::Saga;
use crate::api::saga::types::InMemorySagaRegistry;

/// Factory for creating [`InMemorySagaRegistry`] instances.
pub trait SagaFactory {
    /// Construct an empty in-memory saga registry for saga type `S`.
    fn in_memory_registry<S: Saga>() -> InMemorySagaRegistry<S> {
        InMemorySagaRegistry::new()
    }
}
