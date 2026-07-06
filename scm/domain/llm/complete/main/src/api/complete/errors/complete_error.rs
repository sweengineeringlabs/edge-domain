//! `CompleteError` — 15-variant error taxonomy for LLM completion operations.

/// Comprehensive error taxonomy for LLM completion (renamed from `LlmError` in llmcomplete).
#[derive(Debug, thiserror::Error)]
pub enum CompleteError {
    /// Provider or backend configuration is invalid or missing.
    #[error("configuration error: {0}")]
    Configuration(String),

    /// No provider registered for the requested model or id.
    #[error("provider not found: {0}")]
    ProviderNotFound(String),

    /// The requested model is not available on this provider.
    #[error("model not found: {0}")]
    ModelNotFound(String),

    /// API key or credentials rejected.
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Request rate limit exceeded; optionally carries a retry delay.
    #[error("rate limited")]
    RateLimited {
        /// Milliseconds to wait before retrying, if known.
        retry_after_ms: Option<u64>,
    },

    /// Input exceeds the model's context window.
    #[error("context length exceeded: used {used}, max {max}")]
    ContextLengthExceeded {
        /// Token count that was submitted.
        used: u32,
        /// Maximum tokens the model accepts.
        max: u32,
    },

    /// Response suppressed by a content filter.
    #[error("content filtered: {0}")]
    ContentFiltered(String),

    /// Request parameters are invalid (schema, missing required field, …).
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// Transport-level failure (DNS, TCP, TLS) before the provider responds.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Streaming pipeline broke mid-stream.
    #[error("stream error: {0}")]
    StreamError(String),

    /// Provider did not respond within the allowed window.
    #[error("timeout after {0}ms")]
    Timeout(u64),

    /// Provider returned a well-formed error response.
    #[error("provider error from {provider}: {message}")]
    ProviderError {
        /// Provider name (e.g. `"anthropic"`).
        provider: String,
        /// Human-readable error message from the provider.
        message: String,
    },

    /// JSON serialization or deserialization failed.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Underlying I/O error (file, socket, …).
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    /// A tool-calling loop reached `max_turns` without a terminal finish reason.
    #[error("turn limit exceeded: {max_turns} turns without a terminal finish reason")]
    TurnLimitExceeded {
        /// The configured turn limit that was reached.
        max_turns: u32,
    },
}
