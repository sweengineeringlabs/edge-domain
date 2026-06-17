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

impl ReasoningError {
    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ReasoningError::Timeout { .. } | ReasoningError::BudgetExhausted { .. }
        )
    }

    /// Get human-readable error message
    pub fn message(&self) -> String {
        match self {
            ReasoningError::UnsupportedPattern { pattern } => {
                format!("Unsupported reasoning pattern: {}", pattern)
            }
            ReasoningError::StepFailed { step, reason } => {
                format!("Reasoning step {} failed: {}", step, reason)
            }
            ReasoningError::MaxDepthExceeded { max_depth } => {
                format!("Maximum reasoning depth exceeded: {}", max_depth)
            }
            ReasoningError::BudgetExhausted {
                tokens_used,
                token_limit,
            } => {
                format!("Token budget exhausted: {} / {}", tokens_used, token_limit)
            }
            ReasoningError::ContextTooSmall {
                required,
                available,
            } => {
                format!(
                    "Context window too small: need {}, have {}",
                    required, available
                )
            }
            ReasoningError::Timeout { timeout_secs } => {
                format!("Reasoning timed out after {} seconds", timeout_secs)
            }
            ReasoningError::InvalidState(msg) => format!("Invalid reasoning state: {}", msg),
            ReasoningError::Unknown(msg) => format!("Unknown error: {}", msg),
        }
    }
}
