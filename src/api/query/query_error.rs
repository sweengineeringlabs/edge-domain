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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_error_not_found_message_is_actionable() {
        let e = QueryError::NotFound("order-42".into());
        assert!(e.to_string().contains("order-42"));
    }

    #[test]
    fn test_query_error_invalid_input_message_is_actionable() {
        let e = QueryError::InvalidInput("malformed id".into());
        assert!(e.to_string().contains("malformed id"));
    }
}
