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
    RateLimited { retry_after_ms: Option<u64> },

    /// Context window exceeded
    #[serde(rename = "context_exceeded")]
    ContextWindowExceeded { max_tokens: u32, requested: u32 },

    /// Model not found or unavailable
    #[serde(rename = "model_not_found")]
    ModelNotFound(String),

    /// Provider temporarily unavailable (retryable)
    #[serde(rename = "provider_unavailable")]
    ProviderUnavailable { message: String },

    /// Timeout waiting for response
    #[serde(rename = "timeout")]
    Timeout { duration_ms: u64 },

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
    ToolCallFailed { tool_name: String, reason: String },

    /// Output validation failed
    #[serde(rename = "validation_failed")]
    ValidationFailed(String),

    /// Content filter triggered
    #[serde(rename = "content_filtered")]
    ContentFiltered(String),

    /// Provider quota exceeded (may be retryable)
    #[serde(rename = "quota_exceeded")]
    QuotaExceeded { reset_at_ms: Option<u64> },

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
            ExecutionError::QuotaExceeded { reset_at_ms } => {
                reset_at_ms.map(Duration::from_millis)
            }
            ExecutionError::Timeout { duration_ms } => {
                Some(Duration::from_millis(*duration_ms))
            }
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
