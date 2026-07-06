use serde::{Deserialize, Serialize};

/// Errors that can occur during reasoning processes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReasoningError {
    /// Pattern is not supported
    #[serde(rename = "unsupported_pattern")]
    UnsupportedPattern {
        /// Name of the unsupported pattern
        pattern: String,
    },

    /// Reasoning step failed
    #[serde(rename = "step_failed")]
    StepFailed {
        /// Step index
        step: usize,
        /// Failure reason
        reason: String,
    },

    /// Maximum reasoning depth exceeded
    #[serde(rename = "max_depth_exceeded")]
    MaxDepthExceeded {
        /// Maximum depth allowed
        max_depth: usize,
    },

    /// Token budget exhausted
    #[serde(rename = "budget_exhausted")]
    BudgetExhausted {
        /// Tokens used so far
        tokens_used: usize,
        /// Token limit
        token_limit: usize,
    },

    /// Context window too small
    #[serde(rename = "context_too_small")]
    ContextTooSmall {
        /// Required context size
        required: usize,
        /// Available context size
        available: usize,
    },

    /// Reasoning timed out
    #[serde(rename = "timeout")]
    Timeout {
        /// Timeout duration in seconds
        timeout_secs: u64,
    },

    /// Invalid reasoning state
    #[serde(rename = "invalid_state")]
    InvalidState(String),

    /// Unknown/unclassified error
    #[serde(rename = "unknown")]
    Unknown(String),
}
