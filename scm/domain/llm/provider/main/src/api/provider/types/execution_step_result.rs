use crate::api::provider::types::TokenUsage;
use serde::{Deserialize, Serialize};

/// Result of a single execution step
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionStepResult {
    /// Model's reasoning or thought process
    pub reasoning: String,

    /// Action the model decided to take (tool call name + args)
    pub action: Option<String>,

    /// Confidence level (0.0..=1.0)
    pub confidence: f32,

    /// Token usage for this step (None when the model does not report usage)
    pub tokens_used: Option<TokenUsage>,
}

impl ExecutionStepResult {
    /// Create a new execution step result
    pub fn new(
        reasoning: String,
        action: Option<String>,
        confidence: f32,
        tokens_used: Option<TokenUsage>,
    ) -> Self {
        Self {
            reasoning,
            action,
            confidence,
            tokens_used,
        }
    }

    /// Check if this step resulted in a tool call
    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    /// Check if confidence is high (> 0.8)
    pub fn high_confidence(&self) -> bool {
        self.confidence > 0.8
    }
}
