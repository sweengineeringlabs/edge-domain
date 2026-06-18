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

    /// Token usage for this step
    pub tokens_used: TokenUsage,
}

impl ExecutionStepResult {
    /// Create a new execution step result
    pub fn new(
        reasoning: String,
        action: Option<String>,
        confidence: f32,
        tokens_used: TokenUsage,
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

#[cfg(test)]
mod tests {
    use super::ExecutionStepResult;
    use crate::api::provider::types::TokenUsage;

    #[test]
    fn test_has_action_true_with_action() {
        let r = ExecutionStepResult::new("think".to_string(), Some("tool".to_string()), 0.8, TokenUsage::new(0, 0, 0, 0));
        assert!(r.has_action());
    }

    #[test]
    fn test_has_action_false_without_action() {
        let r = ExecutionStepResult::new("think".to_string(), None, 0.5, TokenUsage::new(0, 0, 0, 0));
        assert!(!r.has_action());
    }

    #[test]
    fn test_high_confidence_above_threshold() {
        let r = ExecutionStepResult::new("x".to_string(), None, 0.9, TokenUsage::new(0, 0, 0, 0));
        assert!(r.high_confidence());
    }

    #[test]
    fn test_execution_step_result_serde_roundtrip() {
        let r = ExecutionStepResult::new("x".to_string(), None, 0.9, TokenUsage::new(1, 1, 0, 0));
        let json = serde_json::to_string(&r).expect("serialize");
        let back: ExecutionStepResult = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.confidence, 0.9);
    }
}
