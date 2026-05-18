//! Error type for [`Command`](super::Command) operations.

/// Error produced by command execution.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    /// The command input was invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// A business rule prevented execution.
    #[error("rule violation: {0}")]
    RuleViolation(String),
    /// The target resource was not found.
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
    fn test_command_error_invalid_input_is_object_safe() {
        let e = CommandError::InvalidInput("bad value".into());
        // Ensure Display is implemented and message is actionable.
        assert!(e.to_string().contains("bad value"));
    }

    #[test]
    fn test_command_error_rule_violation_message_is_actionable() {
        let e = CommandError::RuleViolation("duplicate order".into());
        assert!(e.to_string().contains("duplicate order"));
    }
}
