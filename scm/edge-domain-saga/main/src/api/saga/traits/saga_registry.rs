use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::Saga;

/// Stores live [`Saga`] instances keyed by their `SagaId`.
pub trait SagaRegistry<S: Saga>: Send + Sync {
    /// Register a saga instance under `id`.
    ///
    /// Returns [`SagaError::AlreadyRegistered`] if a saga is already stored
    /// under `id`; the existing instance is left untouched.
    fn register(&mut self, id: S::SagaId, saga: S) -> Result<(), SagaError>;

    /// Borrow the saga registered under `id`.
    ///
    /// Returns [`SagaError::NotFound`] when no saga is registered under `id`.
    fn get(&self, id: &S::SagaId) -> Result<&S, SagaError>;
}
