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
