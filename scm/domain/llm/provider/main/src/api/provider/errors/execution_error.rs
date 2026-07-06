use serde::{Deserialize, Serialize};

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

    /// Transport-level network failure (DNS, TCP, TLS — before the provider responds).
    #[serde(rename = "network_error")]
    NetworkError(String),

    /// Unknown/unclassified error
    #[serde(rename = "unknown")]
    Unknown(String),
}
