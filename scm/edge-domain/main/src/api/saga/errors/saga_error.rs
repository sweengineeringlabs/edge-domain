//! `SagaError` — errors returned by [`SagaRegistry`](crate::SagaRegistry) operations.

use thiserror::Error;

/// Errors that can occur when registering or looking up sagas.
///
/// These cover registry bookkeeping only.  They are never returned by
/// [`Saga::handle`](crate::Saga::handle), which stages commands and cannot fail.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SagaError {
    /// A saga is already registered under the requested id.
    #[error("a saga is already registered under id '{0}'")]
    AlreadyRegistered(String),

    /// No saga is registered under the requested id.
    #[error("no saga is registered under id '{0}'")]
    NotFound(String),
}
