//! Error types for domain execution units.

use thiserror::Error;

/// Errors raised by [`Handler::execute`](crate::Handler::execute).
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

    /// Domain-specific failure that does not fit the above categories.
    #[error("handler error: {0}")]
    Other(String),
}

impl From<crate::api::service::ServiceError> for HandlerError {
    fn from(e: crate::api::service::ServiceError) -> Self {
        use crate::api::service::ServiceError::*;
        match e {
            InvalidRequest(m) => HandlerError::InvalidRequest(m),
            RuleViolation(m)  => HandlerError::FailedPrecondition(m),
            Unavailable(m) | Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::repository_error::RepositoryError> for HandlerError {
    fn from(e: crate::api::repository_error::RepositoryError) -> Self {
        use crate::api::repository_error::RepositoryError::*; // repository not regrouped
        match e {
            NotFound(m)                    => HandlerError::NotFound(m),
            Conflict(m)                    => HandlerError::Conflict(m),
            Unavailable(m) | Internal(m)   => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::command::CommandError> for HandlerError {
    fn from(e: crate::api::command::CommandError) -> Self {
        use crate::api::command::CommandError::*;
        match e {
            InvalidInput(m)  => HandlerError::InvalidRequest(m),
            RuleViolation(m) => HandlerError::FailedPrecondition(m),
            NotFound(m)      => HandlerError::NotFound(m),
            Internal(m)      => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::event::EventError> for HandlerError {
    fn from(e: crate::api::event::EventError) -> Self {
        use crate::api::event::EventError::*;
        match e {
            SerializationFailed(m) | Unavailable(m) | Internal(m) => HandlerError::ExecutionFailed(m),
        }
    }
}

impl From<crate::api::query::QueryError> for HandlerError {
    fn from(e: crate::api::query::QueryError) -> Self {
        use crate::api::query::QueryError::*;
        match e {
            InvalidInput(m) => HandlerError::InvalidRequest(m),
            NotFound(m)     => HandlerError::NotFound(m),
            Internal(m)     => HandlerError::ExecutionFailed(m),
        }
    }
}

impl HandlerError {
    /// Wrap any error as an internal execution failure.
    ///
    /// Consumers use this to convert domain errors into `HandlerError`
    /// without choosing a variant:
    ///
    /// ```rust,ignore
    /// self.complete(req).await.map_err(HandlerError::internal)
    /// ```
    pub fn internal(e: impl ToString) -> Self {
        HandlerError::ExecutionFailed(e.to_string())
    }

    /// Wrap a malformed-input error.
    pub fn invalid(e: impl ToString) -> Self {
        HandlerError::InvalidRequest(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_error_display_contains_message() {
        let err = HandlerError::ExecutionFailed("timeout".to_string());
        assert!(err.to_string().contains("timeout"));
    }

    #[test]
    fn test_handler_error_unhealthy_display() {
        assert_eq!(HandlerError::Unhealthy.to_string(), "handler unhealthy");
    }

    /// @covers: internal
    #[test]
    fn test_internal_produces_execution_failed_variant() {
        let err = HandlerError::internal("connection refused");
        assert!(matches!(err, HandlerError::ExecutionFailed(_)));
        assert!(err.to_string().contains("connection refused"));
    }

    /// @covers: internal
    #[test]
    fn test_internal_usable_as_map_err() {
        let result: Result<(), &str> = Err("boom");
        let mapped = result.map_err(HandlerError::internal);
        assert!(matches!(mapped, Err(HandlerError::ExecutionFailed(_))));
    }

    /// @covers: invalid
    #[test]
    fn test_invalid_produces_invalid_request_variant() {
        let err = HandlerError::invalid("missing field: prompt");
        assert!(matches!(err, HandlerError::InvalidRequest(_)));
        assert!(err.to_string().contains("missing field: prompt"));
    }
}
