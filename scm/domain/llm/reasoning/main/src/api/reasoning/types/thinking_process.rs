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

impl ThinkingProcess {
    /// Create a new thinking process
    pub fn new(id: String, problem: String) -> Self {
        Self {
            id,
            problem,
            steps: vec![],
            total_tokens: 0,
            is_complete: false,
            conclusion: None,
        }
    }

    /// Add a reasoning step
    pub fn add_step(&mut self, step: ReasoningStep) {
        self.total_tokens += step.tokens_consumed;
        self.steps.push(step);
    }

    /// Get the number of reasoning steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get average confidence across all steps
    pub fn average_confidence(&self) -> f32 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.steps.iter().map(|s| s.confidence).sum::<f32>() / self.steps.len() as f32
    }

    /// Mark process as complete with conclusion
    pub fn complete(mut self, conclusion: String) -> Self {
        self.is_complete = true;
        self.conclusion = Some(conclusion);
        self
    }

    /// Get high-confidence steps
    pub fn confident_steps(&self) -> Vec<&ReasoningStep> {
        self.steps.iter().filter(|s| s.is_confident()).collect()
    }

    /// Get low-confidence steps (potential weak points)
    pub fn uncertain_steps(&self) -> Vec<&ReasoningStep> {
        self.steps.iter().filter(|s| !s.is_confident()).collect()
    }
}
