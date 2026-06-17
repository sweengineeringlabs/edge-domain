use serde::{Deserialize, Serialize};

/// Result from a single reasoning step execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepResult {
    /// Whether the step completed successfully
    pub success: bool,

    /// Output from the step
    pub output: String,

    /// Error message if step failed
    pub error: Option<String>,

    /// Duration of step execution in milliseconds
    pub duration_ms: u64,

    /// Whether to continue with next steps
    pub should_continue: bool,

    /// Suggested next action
    pub next_action: Option<String>,
}

impl StepResult {
    /// Create a successful step result
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
            duration_ms: 0,
            should_continue: true,
            next_action: None,
        }
    }

    /// Create a failed step result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            duration_ms: 0,
            should_continue: false,
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
