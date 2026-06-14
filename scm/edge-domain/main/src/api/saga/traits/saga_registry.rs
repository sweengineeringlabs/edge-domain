//! `SagaRegistry` — stores saga instances keyed by their [`SagaId`].
//!
//! [`SagaId`]: crate::Saga::SagaId

use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::Saga;

/// Stores live [`Saga`] instances keyed by their `SagaId`.
///
/// Unlike a string-keyed `HandlerRegistry`, this registry is keyed by the
/// saga's own [`SagaId`](Saga::SagaId) associated type and is generic over a
/// single saga type via `SagaInstance`.  Dispatching the commands a saga emits
/// is the caller's responsibility — the registry only stores and retrieves instances.
pub trait SagaRegistry: Send + Sync {
    /// The concrete saga type stored in this registry.
    type SagaInstance: Saga;

    /// Register a saga instance under `id`.
    ///
    /// Returns [`SagaError::AlreadyRegistered`] if a saga is already stored
    /// under `id`; the existing instance is left untouched.
    fn register(
        &mut self,
        id: <Self::SagaInstance as Saga>::SagaId,
        saga: Self::SagaInstance,
    ) -> Result<(), SagaError>;

    /// Borrow the saga registered under `id`.
    ///
    /// Returns [`SagaError::NotFound`] when no saga is registered under `id`.
    fn get(&self, id: &<Self::SagaInstance as Saga>::SagaId) -> Result<&Self::SagaInstance, SagaError>;
}
