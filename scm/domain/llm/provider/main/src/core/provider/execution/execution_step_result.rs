//! Constructors and accessors for [`ExecutionStepResult`].

use crate::api::{ExecutionStepResult, TokenUsage};

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
            confidence: Self::clamp_confidence(confidence),
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

    /// Clamp a raw confidence score into the valid `0.0..=1.0` range.
    fn clamp_confidence(confidence: f32) -> f32 {
        confidence.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let result = ExecutionStepResult::new("thinking".to_string(), None, 0.5, None);
        assert_eq!(result.reasoning, "thinking");
    }

    /// @covers: has_action
    #[test]
    fn test_has_action_true_when_action_present() {
        let result =
            ExecutionStepResult::new("go".to_string(), Some("search".to_string()), 0.5, None);
        assert!(result.has_action());
    }

    /// @covers: high_confidence
    #[test]
    fn test_high_confidence_false_at_boundary() {
        let result = ExecutionStepResult::new("x".to_string(), None, 0.8, None);
        assert!(!result.high_confidence());
    }

    /// @covers: clamp_confidence
    #[test]
    fn test_clamp_confidence_caps_above_one() {
        assert_eq!(ExecutionStepResult::clamp_confidence(1.5), 1.0);
    }
}
