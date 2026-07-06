//! Inherent methods for [`PromptError`].

use crate::api::PromptError;

impl PromptError {
    /// Check if this error is recoverable (can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PromptError::CacheError(_) | PromptError::RenderFailed(_)
        )
    }

    /// Get human-readable error message
    pub fn message(&self) -> String {
        match self {
            PromptError::InvalidSyntax { message } => {
                format!("Invalid template syntax: {}", message)
            }
            PromptError::MissingVariable { variable_name } => {
                format!("Missing required variable: {}", variable_name)
            }
            PromptError::TypeMismatch {
                variable_name,
                expected,
                actual,
            } => {
                format!(
                    "Type mismatch for '{}': expected {}, got {}",
                    variable_name, expected, actual
                )
            }
            PromptError::InvalidValue {
                variable_name,
                reason,
            } => {
                format!("Invalid value for '{}': {}", variable_name, reason)
            }
            PromptError::IncompleteContext { missing_variables } => {
                format!(
                    "Incomplete context: missing {}",
                    missing_variables.join(", ")
                )
            }
            PromptError::RenderFailed(msg) => Self::labeled("Rendering failed", msg),
            PromptError::CacheError(msg) => Self::labeled("Cache error", msg),
            PromptError::TokenizationError(msg) => Self::labeled("Tokenization error", msg),
            PromptError::Unknown(msg) => Self::labeled("Unknown error", msg),
        }
    }

    /// Join a label and detail into a single human-readable message.
    fn labeled(label: &str, detail: &str) -> String {
        format!("{label}: {detail}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_recoverable
    #[test]
    fn test_is_recoverable_cache_error_is_true() {
        assert!(PromptError::CacheError("x".into()).is_recoverable());
    }

    /// @covers: is_recoverable
    #[test]
    fn test_is_recoverable_invalid_syntax_is_false() {
        assert!(!PromptError::InvalidSyntax {
            message: "x".into()
        }
        .is_recoverable());
    }

    /// @covers: message
    #[test]
    fn test_message_missing_variable_includes_name() {
        let msg = PromptError::MissingVariable {
            variable_name: "name".into(),
        }
        .message();
        assert!(msg.contains("name"));
    }

    /// @covers: labeled
    #[test]
    fn test_labeled_joins_label_and_detail() {
        assert_eq!(
            PromptError::labeled("Cache error", "boom"),
            "Cache error: boom"
        );
    }
}
