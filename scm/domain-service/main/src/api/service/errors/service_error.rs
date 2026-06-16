//! Error type for [`Service`](super::super::traits::Service) operations.

/// Error produced by service execution.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    /// The service request was invalid.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// A business rule prevented execution.
    #[error("business rule violation: {0}")]
    RuleViolation(String),
    /// The target resource was not found.
    #[error("not found: {0}")]
    NotFound(String),
    /// The service is temporarily unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
