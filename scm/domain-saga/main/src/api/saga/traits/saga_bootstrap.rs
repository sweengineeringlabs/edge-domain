use crate::api::saga::traits::Saga;
use crate::api::saga::types::{InMemorySagaStore, NoopSaga, NoopSagaCommand, NoopSagaEvent, StdSagaFactory};

/// Bootstrap constructor contract for saga infrastructure instances.
pub trait SagaBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "saga"
    }

    /// Construct an empty in-memory saga store for saga type `S`.
    fn in_memory_store<S: Saga>() -> InMemorySagaStore<S> where Self: Sized {
        InMemorySagaStore::new()
    }

    /// Construct a [`NoopSaga`] that never processes events.
    ///
    /// Useful as a placeholder where a concrete `Saga` is required but
    /// no actual event processing should occur.
    fn noop() -> NoopSaga where Self: Sized {
        NoopSaga::default()
    }

    /// Construct a [`NoopSagaEvent`] — a zero-payload event for noop sagas.
    fn noop_event() -> NoopSagaEvent where Self: Sized {
        NoopSagaEvent
    }

    /// Construct a [`NoopSagaCommand`] — a no-op command for noop sagas.
    fn noop_command() -> NoopSagaCommand where Self: Sized {
        NoopSagaCommand
    }

    /// Return the standard saga factory implementation.
    fn std_factory() -> StdSagaFactory where Self: Sized {
        StdSagaFactory
    }
}
