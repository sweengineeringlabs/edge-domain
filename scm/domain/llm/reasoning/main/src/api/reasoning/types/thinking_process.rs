use crate::api::reasoning::types::ReasoningStep;
use serde::{Deserialize, Serialize};

/// Complete thinking/reasoning process
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThinkingProcess {
    /// Unique ID for this reasoning process
    pub id: String,

    /// Problem statement
    pub problem: String,

    /// All reasoning steps in order
    pub steps: Vec<ReasoningStep>,

    /// Total tokens consumed
    pub total_tokens: usize,

    /// Whether the process is complete
    pub is_complete: bool,

    /// Final conclusion or answer
    pub conclusion: Option<String>,
}
