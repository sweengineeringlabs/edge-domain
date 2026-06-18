use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Comprehensive error taxonomy for LLM provider execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExecutionError {
    /// Authentication/authorization failure
    #[serde(rename = "auth_failed")]
    AuthenticationFailed(String),

    /// Rate limit exceeded (may be retryable)
    #[serde(rename = "rate_limited")]
    RateLimited {
        /// Milliseconds to wait before retrying
        retry_after_ms: Option<u64>,
    },

    /// Context window exceeded
    #[serde(rename = "context_exceeded")]
    ContextWindowExceeded {
        /// Maximum context tokens allowed
        max_tokens: u32,
        /// Tokens that were requested
        requested: u32,
    },

    /// Model not found or unavailable
    #[serde(rename = "model_not_found")]
    ModelNotFound(String),

    /// Provider temporarily unavailable (retryable)
    #[serde(rename = "provider_unavailable")]
    ProviderUnavailable {
        /// Error message from provider
        message: String,
    },

    /// Timeout waiting for response
    #[serde(rename = "timeout")]
    Timeout {
        /// Timeout duration in milliseconds
        duration_ms: u64,
    },

    /// Invalid request parameters
    #[serde(rename = "invalid_request")]
    InvalidRequest(String),

    /// Streaming error mid-stream
    #[serde(rename = "streaming_error")]
    StreamingError(String),

    /// Cache operation failed
    #[serde(rename = "cache_error")]
    CacheError(String),

    /// Tool/function call failed
    #[serde(rename = "tool_call_failed")]
    ToolCallFailed {
        /// Name of the tool that failed
        tool_name: String,
        /// Reason for failure
        reason: String,
    },

    /// Output validation failed
    #[serde(rename = "validation_failed")]
    ValidationFailed(String),

    /// Content filter triggered
    #[serde(rename = "content_filtered")]
    ContentFiltered(String),

    /// Provider quota exceeded (may be retryable)
    #[serde(rename = "quota_exceeded")]
    QuotaExceeded {
        /// Unix timestamp when quota resets (if known)
        reset_at_ms: Option<u64>,
    },

    /// Unknown/unclassified error
    #[serde(rename = "unknown")]
    Unknown(String),
}

impl ExecutionError {
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ExecutionError::RateLimited { .. }
                | ExecutionError::ProviderUnavailable { .. }
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
            ExecutionError::AuthenticationFailed(msg) => format!("Authentication failed: {}", msg),
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
            ExecutionError::ModelNotFound(model) => format!("Model not found: {}", model),
            ExecutionError::ProviderUnavailable { message } => {
                format!("Provider unavailable: {}", message)
            }
            ExecutionError::Timeout { duration_ms } => {
                format!("Timeout after {}ms", duration_ms)
            }
            ExecutionError::InvalidRequest(msg) => format!("Invalid request: {}", msg),
            ExecutionError::StreamingError(msg) => format!("Streaming error: {}", msg),
            ExecutionError::CacheError(msg) => format!("Cache error: {}", msg),
            ExecutionError::ToolCallFailed { tool_name, reason } => {
                format!("Tool call failed ({}): {}", tool_name, reason)
            }
            ExecutionError::ValidationFailed(msg) => format!("Validation failed: {}", msg),
            ExecutionError::ContentFiltered(msg) => format!("Content filtered: {}", msg),
            ExecutionError::QuotaExceeded { reset_at_ms } => {
                format!("Quota exceeded (reset at {:?})", reset_at_ms)
            }
            ExecutionError::Unknown(msg) => format!("Unknown error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionError;
    use std::time::Duration;

    #[test]
    fn test_is_retryable_rate_limited() {
        let err = ExecutionError::RateLimited { retry_after_ms: Some(1000) };
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_auth_failed_false() {
        let err = ExecutionError::AuthenticationFailed("bad key".to_string());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_retry_after_returns_duration() {
        let err = ExecutionError::RateLimited { retry_after_ms: Some(2000) };
        assert_eq!(err.retry_after(), Some(Duration::from_millis(2000)));
    }

    #[test]
    fn test_retry_after_none_for_auth() {
        let err = ExecutionError::AuthenticationFailed("x".to_string());
        assert_eq!(err.retry_after(), None);
    }

    #[test]
    fn test_message_mentions_context_window() {
        let err = ExecutionError::ContextWindowExceeded { max_tokens: 8192, requested: 9000 };
        assert!(err.message().contains("Context window"));
    }

    #[test]
    fn test_execution_error_serde_roundtrip() {
        let err = ExecutionError::Timeout { duration_ms: 30_000 };
        let json = serde_json::to_string(&err).expect("serialize");
        let back: ExecutionError = serde_json::from_str(&json).expect("deserialize");
        assert!(matches!(back, ExecutionError::Timeout { .. }));
    }
}
