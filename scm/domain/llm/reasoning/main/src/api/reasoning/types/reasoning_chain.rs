use crate::api::reasoning::types::ThinkingProcess;
use serde::{Deserialize, Serialize};

/// Chain of multiple reasoning processes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningChain {
    /// Unique ID for this reasoning chain
    pub id: String,

    /// Ordered list of reasoning processes
    pub processes: Vec<ThinkingProcess>,

    /// Whether chain is complete
    pub is_complete: bool,

    /// Final answer after all processes
    pub final_answer: Option<String>,
}
