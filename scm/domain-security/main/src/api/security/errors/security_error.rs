//! [`SecurityError`] — errors produced by security context operations.

use thiserror::Error;

/// Error returned when constructing or enforcing a [`SecurityContext`](crate::SecurityContext).
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum SecurityError {
    /// No claims were provided when at least one was required.
    #[error("security context requires at least one claim")]
    MissingClaims,
    /// The principal identity string is empty.
    #[error("principal id must not be empty")]
    EmptyPrincipalId,
    /// The security context is not authenticated.
    #[error("security context is not authenticated")]
    Unauthenticated,
}
