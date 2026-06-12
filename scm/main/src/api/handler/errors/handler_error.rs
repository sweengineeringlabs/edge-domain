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
/// `PermissionDenied` → 403/PERMISSION_DENIED, `Conflict` → 409/ALREADY_EXISTS,
/// `Timeout` → 504/DEADLINE_EXCEEDED.
///
/// # Domain error conversion
///
/// Convert domain-layer errors (`CommandError`, `QueryError`, `RepositoryError`,
/// etc.) at the handler call site using [`HandlerError::internal`] or
/// [`HandlerError::invalid`]:
///
/// ```rust,ignore
/// bus.dispatch(cmd).await.map_err(HandlerError::internal)?;
/// ```
///
/// # Examples
///
/// ```rust
/// use edge_domain::HandlerError;
///
/// let e = HandlerError::internal("database unavailable");
/// assert!(matches!(e, HandlerError::ExecutionFailed(_)));
///
/// let e = HandlerError::invalid("id must be a UUID");
/// assert!(matches!(e, HandlerError::InvalidRequest(_)));
///
/// let e = HandlerError::NotFound("order-123".to_string());
/// assert!(e.to_string().contains("order-123"));
///
/// let e = HandlerError::Unauthorized("JWT expired".to_string());
/// let status = match &e {
///     HandlerError::Unauthorized(_)     => 401,
///     HandlerError::PermissionDenied(_) => 403,
///     HandlerError::NotFound(_)         => 404,
///     HandlerError::Conflict(_)         => 409,
///     HandlerError::Timeout(_)          => 504,
///     HandlerError::Skipped             => 200,
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

    /// Caller asked for an operation the handler cannot perform in its current
    /// state. Maps to gRPC `FAILED_PRECONDITION`.
    #[error("failed precondition: {0}")]
    FailedPrecondition(String),

    /// The caller is not authenticated. Maps to HTTP 401 / gRPC `UNAUTHENTICATED`.
    #[error("unauthorized: {0}")]
    Unauthorized(String),

    /// The caller is authenticated but not permitted. Maps to HTTP 403 / gRPC `PERMISSION_DENIED`.
    #[error("permission denied: {0}")]
    PermissionDenied(String),

    /// The handler did not complete within its configured deadline.
    /// Maps to HTTP 504 / gRPC `DEADLINE_EXCEEDED`.
    ///
    /// Both `TimeoutPolicy::FailClosed` and `TimeoutPolicy::FailOpen` surface
    /// this variant so callers can distinguish a deadline from a genuine failure.
    #[error("timeout: {0}")]
    Timeout(String),

    /// The handler was disabled and skipped execution. Pipeline-internal sentinel —
    /// not a real failure. RFC-001 `Pipeline` treats this as "continue to next stage".
    #[error("handler skipped")]
    Skipped,
}

impl HandlerError {
    /// Wrap any error as an internal execution failure.
    ///
    /// Use as `.map_err(HandlerError::internal)` to convert any `Display` error
    /// without picking a variant at the call site.
    ///
    /// ```rust
    /// use edge_domain::HandlerError;
    /// let e = HandlerError::internal("pool exhausted");
    /// assert!(matches!(e, HandlerError::ExecutionFailed(_)));
    /// ```
    pub fn internal(e: impl ToString) -> Self {
        HandlerError::ExecutionFailed(e.to_string())
    }

    /// Wrap a malformed-input error as [`HandlerError::InvalidRequest`].
    pub fn invalid(e: impl ToString) -> Self {
        HandlerError::InvalidRequest(e.to_string())
    }
}
