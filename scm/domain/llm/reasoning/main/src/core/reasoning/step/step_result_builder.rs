//! Constructors and builder methods for [`StepResultBuilder`].

use crate::api::{StepResult, StepResultBuilder};

impl Default for StepResultBuilder {
    fn default() -> Self {
        Self {
            success: true,
            output: String::new(),
            error: None,
            duration_ms: 0,
            should_continue: true,
            next_action: None,
        }
    }
}

impl StepResultBuilder {
    /// Start a new builder with default (successful) values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the success flag.
    pub fn success(mut self, value: bool) -> Self {
        self.success = value;
        self
    }

    /// Set the step output.
    pub fn output(mut self, value: String) -> Self {
        self.output = value;
        self
    }

    /// Set the error message (also marks the result as failed).
    pub fn error(mut self, value: String) -> Self {
        self.success = false;
        self.should_continue = false;
        self.error = Some(value);
        self
    }

    /// Set the execution duration in milliseconds.
    pub fn duration_ms(mut self, value: u64) -> Self {
        self.duration_ms = value;
        self
    }

    /// Set whether reasoning should continue.
    pub fn should_continue(mut self, value: bool) -> Self {
        self.should_continue = value;
        self
    }

    /// Set the suggested next action.
    pub fn next_action(mut self, value: String) -> Self {
        self.next_action = Some(value);
        self
    }

    /// Build the [`StepResult`].
    pub fn build(self) -> StepResult {
        StepResult {
            success: self.success,
            output: self.output,
            error: self.error,
            duration_ms: self.duration_ms,
            should_continue: self.should_continue,
            next_action: self.next_action,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: default
    #[test]
    fn test_default_is_successful() {
        assert!(StepResultBuilder::default().success);
    }

    /// @covers: new
    #[test]
    fn test_new_matches_default() {
        let builder = StepResultBuilder::new();
        assert!(builder.success);
    }

    /// @covers: error
    #[test]
    fn test_error_marks_result_failed() {
        let result = StepResultBuilder::new().error("bad".to_string()).build();
        assert!(!result.success);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_step_result() {
        let result = StepResultBuilder::new().output("x".to_string()).build();
        assert_eq!(result.output, "x");
    }

    /// @covers: success
    #[test]
    fn test_success_sets_value() {
        let result = StepResultBuilder::new().success(false).build();
        assert!(!result.success);
    }

    /// @covers: output
    #[test]
    fn test_output_sets_value() {
        let result = StepResultBuilder::new().output("hi".to_string()).build();
        assert_eq!(result.output, "hi");
    }

    /// @covers: duration_ms
    #[test]
    fn test_duration_ms_sets_value() {
        let result = StepResultBuilder::new().duration_ms(250).build();
        assert_eq!(result.duration_ms, 250);
    }

    /// @covers: should_continue
    #[test]
    fn test_should_continue_sets_value() {
        let result = StepResultBuilder::new().should_continue(false).build();
        assert!(!result.should_continue);
    }

    /// @covers: next_action
    #[test]
    fn test_next_action_sets_value() {
        let result = StepResultBuilder::new()
            .next_action("retry".to_string())
            .build();
        assert_eq!(result.next_action, Some("retry".to_string()));
    }
}
