//! [`StdSagaFactory`] — the standard implementation marker for [`SagaFactory`](crate::api::saga::traits::SagaFactory).

/// Standard implementation marker for [`SagaFactory`](crate::api::saga::traits::SagaFactory).
///
/// Callers invoke associated functions on this type to create saga infrastructure,
/// e.g. `StdSagaFactory::in_memory_registry::<MySaga>()`.
pub struct StdSagaFactory;
