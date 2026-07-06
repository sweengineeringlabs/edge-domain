//! Constructors and accessors for [`StepResult`].

use crate::api::StepResult;

impl StepResult {
    /// Create a successful step result
    pub fn success(output: String) -> Self {
        Self::assemble(true, output, None, true)
    }

    /// Create a failed step result
    pub fn failure(error: String) -> Self {
        Self::assemble(false, String::new(), Some(error), false)
    }

    /// Assemble a `StepResult` from its variable fields, defaulting `duration_ms`
    /// and `next_action` — shared by [`Self::success`] and [`Self::failure`].
    fn assemble(
        success: bool,
        output: String,
        error: Option<String>,
        should_continue: bool,
    ) -> Self {
        Self {
            success,
            output,
            error,
            duration_ms: 0,
            should_continue,
            next_action: None,
        }
    }

    /// Set execution duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set whether to continue
    pub fn with_continuation(mut self, should_continue: bool) -> Self {
        self.should_continue = should_continue;
        self
    }

    /// Set suggested next action
    pub fn with_next_action(mut self, action: String) -> Self {
        self.next_action = Some(action);
        self
    }

    /// Check if step execution was fast (< 1 second)
    pub fn was_fast(&self) -> bool {
        self.duration_ms < 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: success
    #[test]
    fn test_success_sets_success_true() {
        assert!(StepResult::success("ok".to_string()).success);
    }

    /// @covers: failure
    #[test]
    fn test_failure_sets_success_false() {
        assert!(!StepResult::failure("bad".to_string()).success);
    }

    /// @covers: with_duration
    #[test]
    fn test_with_duration_sets_value() {
        let result = StepResult::success("ok".to_string()).with_duration(500);
        assert_eq!(result.duration_ms, 500);
    }

    /// @covers: with_continuation
    #[test]
    fn test_with_continuation_sets_value() {
        let result = StepResult::success("ok".to_string()).with_continuation(false);
        assert!(!result.should_continue);
    }

    /// @covers: with_next_action
    #[test]
    fn test_with_next_action_sets_value() {
        let result = StepResult::success("ok".to_string()).with_next_action("go".to_string());
        assert_eq!(result.next_action, Some("go".to_string()));
    }

    /// @covers: was_fast
    #[test]
    fn test_was_fast_true_under_one_second() {
        let result = StepResult::success("ok".to_string()).with_duration(100);
        assert!(result.was_fast());
    }

    /// @covers: assemble
    #[test]
    fn test_assemble_defaults_duration_and_next_action() {
        let result = StepResult::assemble(true, "ok".to_string(), None, true);
        assert_eq!(result.duration_ms, 0);
        assert_eq!(result.next_action, None);
    }
}
