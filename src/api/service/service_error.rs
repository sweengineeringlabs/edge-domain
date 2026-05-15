//! Error type for [`Service`](super::Service) operations.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_error_invalid_request_message_is_actionable() {
        let e = ServiceError::InvalidRequest("missing customer_id".into());
        assert!(e.to_string().contains("missing customer_id"));
    }

    #[test]
    fn test_service_error_rule_violation_message_is_actionable() {
        let e = ServiceError::RuleViolation("quota exceeded".into());
        assert!(e.to_string().contains("quota exceeded"));
    }
}
