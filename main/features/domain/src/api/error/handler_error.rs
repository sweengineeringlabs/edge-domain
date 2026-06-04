//! Error types for domain execution units.

use thiserror::Error;

/// Errors raised by [`Handler::execute`](crate::Handler::execute).
///
/// All variants carry a human-readable message. Use the helper constructors
/// [`HandlerError::internal`] and [`HandlerError::invalid`] to wrap arbitrary
/// errors without choosing a variant at the call site.
///
/// Map to HTTP/gRPC status codes at the ingress layer:
/// `NotFound` → 404/NOT_FOUND, `Unauthorized` → 401/UNAUTHENTICATED,
/// `PermissionDenied` → 403/PERMISSION_DENIED, `Conflict` → 409/ALREADY_EXISTS.
///
/// # Examples
///
/// ```rust
/// use edge_domain::HandlerError;
///
/// // Constructor helpers.
/// let e = HandlerError::internal("database unavailable");
/// assert!(e.to_string().contains("database unavailable"));
///
/// let e = HandlerError::invalid("id must be a UUID");
/// assert!(matches!(e, HandlerError::InvalidRequest(_)));
///
/// // Direct variant construction.
/// let e = HandlerError::NotFound("order-123".to_string());
/// assert!(e.to_string().contains("order-123"));
///
/// // Exhaustive match.
/// let e = HandlerError::Unauthorized("JWT expired".to_string());
/// let status = match &e {
///     HandlerError::Unauthorized(_)    => 401,
///     HandlerError::PermissionDenied(_) => 403,
///     HandlerError::NotFound(_)         => 404,
///     HandlerError::Conflict(_)         => 409,
///     HandlerError::InvalidRequest(_)
///     | HandlerError::FailedPrecondition(_) => 400,
///     HandlerError::Unsupported(_)
///     | HandlerError::ExecutionFailed(_)
///     | HandlerError::Unhealthy         => 500,
/// };
/// assert_eq!(status, 401);
/// ```
#[derive(Debug, Error)]
pub enum HandlerError {
    /// The handler was asked to do something it does not support.
    #[error("unsupported operation: {0}")]
    Unsupported(String),

    /// Handler input was malformed.
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// The operation would create a duplicate or violate a uniqueness constraint.
    #[error("conflict: {0}")]
    Conflict(String),

    /// Handler ran to completion but the execution did not succeed.
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    /// The handler is currently unhealthy and refused the request.
    #[error("handler unhealthy")]
    Unhealthy,

    /// Caller asked for an operation the handler cannot perform in its
    /// current state — distinct from `InvalidRequest` (request shape is
    /// fine; system state is not). Maps to gRPC `FAILED_PRECONDITION`.
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),

    /// The caller is not authenticated. Maps to HTTP 401 / gRPC `UNAUTHENTICATED`.
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    /// The caller is authenticated but not permitted. Maps to HTTP 403 / gRPC `PERMISSION_DENIED`.
    #[error("permission denied: {0}")]
    PermissionDenied(String),
}

impl From<crate::api::service::ServiceError> for HandlerError {
    fn from(e: crate::api::service::ServiceError) -> Self {
        use crate::api::service::ServiceError::*;
        match e {
            InvalidRequest(m) => HandlerError::InvalidRequest(m),
            RuleViolation(m) => HandlerError::FailedPrecondition(m),
            NotFound(m) => HandlerError::NotFound(m),
            Unavailable(m) | Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::error::RepositoryError> for HandlerError {
    fn from(e: crate::api::error::RepositoryError) -> Self {
        use crate::api::error::RepositoryError::*;
        match e {
            NotFound(m) => HandlerError::NotFound(m),
            Conflict(m) => HandlerError::Conflict(m),
            Unavailable(m) | Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::command::CommandError> for HandlerError {
    fn from(e: crate::api::command::CommandError) -> Self {
        use crate::api::command::CommandError::*;
        match e {
            InvalidInput(m) => HandlerError::InvalidRequest(m),
            RuleViolation(m) => HandlerError::FailedPrecondition(m),
            NotFound(m) => HandlerError::NotFound(m),
            Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::event::EventError> for HandlerError {
    fn from(e: crate::api::event::EventError) -> Self {
        use crate::api::event::EventError::*;
        match e {
            SerializationFailed(m) | Unavailable(m) => HandlerError::ExecutionFailed(m),
            BroadcastLagged(n) => {
                HandlerError::ExecutionFailed(format!("broadcast lagged: {n} messages dropped"))
            }
        }
    }
}

impl From<crate::api::query::QueryError> for HandlerError {
    fn from(e: crate::api::query::QueryError) -> Self {
        use crate::api::query::QueryError::*;
        match e {
            InvalidInput(m) => HandlerError::InvalidRequest(m),
            NotFound(m) => HandlerError::NotFound(m),
            Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl HandlerError {
    /// Wrap any error as an internal execution failure.
    ///
    /// Use as `.map_err(HandlerError::internal)` to convert any `Display` error
    /// into `HandlerError::ExecutionFailed` without picking a variant at the call site.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use edge_domain::HandlerError;
    /// let e = HandlerError::internal("pool exhausted");
    /// assert!(matches!(e, HandlerError::ExecutionFailed(_)));
    /// ```
    pub fn internal(e: impl ToString) -> Self {
        HandlerError::ExecutionFailed(e.to_string())
    }

    /// Wrap a malformed-input error.
    pub fn invalid(e: impl ToString) -> Self {
        HandlerError::InvalidRequest(e.to_string())
    }
}
