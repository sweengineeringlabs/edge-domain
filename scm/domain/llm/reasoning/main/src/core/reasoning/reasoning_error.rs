//! Behaviour for [`ReasoningError`].

use std::fmt;

use crate::api::ReasoningError;

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
            ReasoningError::InvalidState(msg) => Self::labeled("Invalid reasoning state", msg),
            ReasoningError::Unknown(msg) => Self::labeled("Unknown error", msg),
        }
    }

    /// Format a `label: detail` message, shared by the single-message variants.
    fn labeled(label: &str, detail: &str) -> String {
        format!("{}: {}", label, detail)
    }
}

impl fmt::Display for ReasoningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ReasoningError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_recoverable
    #[test]
    fn test_is_recoverable_true_for_timeout() {
        let err = ReasoningError::Timeout { timeout_secs: 5 };
        assert!(err.is_recoverable());
    }

    /// @covers: is_recoverable
    #[test]
    fn test_is_recoverable_false_for_invalid_state() {
        let err = ReasoningError::InvalidState("bad".to_string());
        assert!(!err.is_recoverable());
    }

    /// @covers: message
    #[test]
    fn test_message_includes_pattern_name() {
        let err = ReasoningError::UnsupportedPattern {
            pattern: "graph".to_string(),
        };
        assert!(err.message().contains("graph"));
    }

    /// @covers: labeled
    #[test]
    fn test_labeled_formats_with_colon_separator() {
        assert_eq!(ReasoningError::labeled("Label", "detail"), "Label: detail");
    }

    /// @covers: fmt
    #[test]
    fn test_display_matches_message_happy() {
        let err = ReasoningError::Timeout { timeout_secs: 5 };
        assert_eq!(err.to_string(), err.message());
    }

    /// @covers: fmt
    #[test]
    fn test_reasoning_error_usable_as_std_error_trait_object_edge() {
        let err = ReasoningError::InvalidState("bad".to_string());
        let as_error: &dyn std::error::Error = &err;
        assert_eq!(as_error.to_string(), err.message());
    }
}
