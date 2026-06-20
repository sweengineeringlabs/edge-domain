//! [`StdSagaFactory`] — the standard implementation marker for [`SagaBootstrap`](crate::api::saga::traits::SagaBootstrap).

/// Standard implementation marker for [`SagaBootstrap`](crate::api::saga::traits::SagaBootstrap).
///
/// Callers invoke associated functions on this type to create saga infrastructure,
/// e.g. `StdSagaFactory::in_memory_store::<MySaga>()`.
pub struct StdSagaFactory;
