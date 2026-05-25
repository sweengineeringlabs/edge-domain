//! Error type for [`Query`](super::Query) operations.
//!
//! Queries are read-only — `RuleViolation` is intentionally absent because
//! business rules are a write concern enforced by [`Command`](crate::api::command::Command).

/// Error produced by query execution.
#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// The query input was invalid (e.g. malformed ID).
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}


