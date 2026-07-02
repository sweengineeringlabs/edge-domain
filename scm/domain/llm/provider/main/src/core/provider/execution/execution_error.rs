//! Behaviour for [`ExecutionError`].

use std::time::Duration;

use crate::api::ExecutionError;

impl ExecutionError {
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ExecutionError::RateLimited { .. }
                | ExecutionError::ProviderUnavailable { .. }
                | ExecutionError::NetworkError(_)
                | ExecutionError::Timeout { .. }
                | ExecutionError::QuotaExceeded { .. }
        )
    }

    /// Get retry-after duration if available
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            ExecutionError::RateLimited { retry_after_ms } => {
                retry_after_ms.map(Duration::from_millis)
            }
            ExecutionError::QuotaExceeded { reset_at_ms } => reset_at_ms.map(Duration::from_millis),
            ExecutionError::Timeout { duration_ms } => Some(Duration::from_millis(*duration_ms)),
            _ => None,
        }
    }

    /// Get error message
    pub fn message(&self) -> String {
        match self {
            ExecutionError::AuthenticationFailed(msg) => {
                Self::labeled("Authentication failed", msg)
            }
            ExecutionError::RateLimited { retry_after_ms } => {
                format!("Rate limited (retry after {:?})", retry_after_ms)
            }
            ExecutionError::ContextWindowExceeded {
                max_tokens,
                requested,
            } => {
                format!(
                    "Context window exceeded: requested {} > max {}",
                    requested, max_tokens
                )
            }
            ExecutionError::ModelNotFound(model) => Self::labeled("Model not found", model),
            ExecutionError::ProviderUnavailable { message } => {
                Self::labeled("Provider unavailable", message)
            }
            ExecutionError::Timeout { duration_ms } => {
                format!("Timeout after {}ms", duration_ms)
            }
            ExecutionError::InvalidRequest(msg) => Self::labeled("Invalid request", msg),
            ExecutionError::StreamingError(msg) => Self::labeled("Streaming error", msg),
            ExecutionError::CacheError(msg) => Self::labeled("Cache error", msg),
            ExecutionError::ToolCallFailed { tool_name, reason } => {
                format!("Tool call failed ({}): {}", tool_name, reason)
            }
            ExecutionError::ValidationFailed(msg) => Self::labeled("Validation failed", msg),
            ExecutionError::ContentFiltered(msg) => Self::labeled("Content filtered", msg),
            ExecutionError::QuotaExceeded { reset_at_ms } => {
                format!("Quota exceeded (reset at {:?})", reset_at_ms)
            }
            ExecutionError::NetworkError(msg) => Self::labeled("Network error", msg),
            ExecutionError::Unknown(msg) => Self::labeled("Unknown error", msg),
        }
    }

    /// Format a `"<label>: <detail>"` message shared by the single-message variants.
    fn labeled(label: &str, detail: &str) -> String {
        format!("{label}: {detail}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_retryable
    #[test]
    fn test_is_retryable_true_for_rate_limited() {
        let err = ExecutionError::RateLimited {
            retry_after_ms: Some(100),
        };
        assert!(err.is_retryable());
    }

    /// @covers: is_retryable
    #[test]
    fn test_is_retryable_false_for_invalid_request() {
        let err = ExecutionError::InvalidRequest("bad".to_string());
        assert!(!err.is_retryable());
    }

    /// @covers: retry_after
    #[test]
    fn test_retry_after_returns_duration_for_timeout() {
        let err = ExecutionError::Timeout { duration_ms: 500 };
        assert_eq!(err.retry_after(), Some(Duration::from_millis(500)));
    }

    /// @covers: message
    #[test]
    fn test_message_includes_model_name() {
        let err = ExecutionError::ModelNotFound("gpt-5".to_string());
        assert!(err.message().contains("gpt-5"));
    }

    /// @covers: labeled
    #[test]
    fn test_labeled_joins_label_and_detail() {
        let err = ExecutionError::InvalidRequest("bad input".to_string());
        assert_eq!(err.message(), "Invalid request: bad input");
    }
}
