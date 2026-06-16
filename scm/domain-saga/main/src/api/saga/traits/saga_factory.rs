use crate::api::saga::traits::Saga;
use crate::api::saga::types::{InMemorySagaStore, NoopSaga, NoopSagaCommand, NoopSagaEvent, StdSagaFactory};

/// Factory for creating saga infrastructure instances.
pub trait SagaFactory {
    /// Construct an empty in-memory saga store for saga type `S`.
    fn in_memory_store<S: Saga>() -> InMemorySagaStore<S> {
        InMemorySagaStore::new()
    }

    /// Construct a [`NoopSaga`] that never processes events.
    ///
    /// Useful as a placeholder where a concrete `Saga` is required but
    /// no actual event processing should occur.
    fn noop() -> NoopSaga {
        NoopSaga::default()
    }

    /// Construct a [`NoopSagaEvent`] — a zero-payload event for noop sagas.
    fn noop_event() -> NoopSagaEvent {
        NoopSagaEvent
    }

    /// Construct a [`NoopSagaCommand`] — a no-op command for noop sagas.
    fn noop_command() -> NoopSagaCommand {
        NoopSagaCommand
    }

    /// Return the standard saga factory implementation.
    fn std_factory() -> StdSagaFactory {
        StdSagaFactory
    }
}
