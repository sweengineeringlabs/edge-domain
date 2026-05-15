//! Error type for [`Command`](super::command::Command) and [`Query`](super::query::Query) operations.

/// Error produced by command or query execution.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    /// The command or query input was invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// A business rule prevented execution.
    #[error("rule violation: {0}")]
    RuleViolation(String),
    /// The target resource was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
