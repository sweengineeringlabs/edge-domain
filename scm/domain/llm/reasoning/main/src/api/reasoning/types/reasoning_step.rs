use serde::{Deserialize, Serialize};

/// A single step in a reasoning process
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step index (0-based)
    pub index: usize,

    /// The reasoning content (thought, analysis, etc.)
    pub content: String,

    /// Type of step (e.g., "analysis", "hypothesis", "synthesis")
    pub step_type: String,

    /// Confidence level (0.0..=1.0)
    pub confidence: f32,

    /// Tokens consumed by this step
    pub tokens_consumed: usize,

    /// Parent step index (if part of tree reasoning)
    pub parent_step: Option<usize>,

    /// Child step indices (if branching)
    pub child_steps: Vec<usize>,
}
