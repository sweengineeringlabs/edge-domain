//! Error type for [`Service`](super::service::Service) operations.

/// Error produced by domain service operations.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    /// The request was invalid.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// A business rule was violated.
    #[error("business rule violation: {0}")]
    RuleViolation(String),
    /// The service is temporarily unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// An unexpected internal error occurred.
    #[error("internal: {0}")]
    Internal(String),
}
