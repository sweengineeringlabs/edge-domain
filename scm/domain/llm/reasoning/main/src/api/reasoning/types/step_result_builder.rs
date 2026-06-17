//! `StepResultBuilder` — fluent builder for [`StepResult`].

use crate::api::reasoning::types::StepResult;

/// Fluent builder for [`StepResult`].
#[derive(Clone, Debug)]
pub struct StepResultBuilder {
    success: bool,
    output: String,
    error: Option<String>,
    duration_ms: u64,
    should_continue: bool,
    next_action: Option<String>,
}

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
