//! Constructors and accessors for [`ReasoningStep`].

use crate::api::ReasoningStep;

impl ReasoningStep {
    /// Create a new reasoning step
    pub fn new(index: usize, content: String, step_type: String) -> Self {
        Self {
            index,
            content,
            step_type,
            confidence: 0.5,
            tokens_consumed: 0,
            parent_step: None,
            child_steps: vec![],
        }
    }

    /// Check if this step has high confidence
    pub fn is_confident(&self) -> bool {
        self.confidence > 0.8
    }

    /// Check if this step is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        self.child_steps.is_empty()
    }

    /// Check if this step is a root (no parent)
    pub fn is_root(&self) -> bool {
        self.parent_step.is_none()
    }

    /// Set confidence level
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = Self::clamp_confidence(confidence);
        self
    }

    /// Clamp a raw confidence value into the valid `0.0..=1.0` range.
    fn clamp_confidence(confidence: f32) -> f32 {
        confidence.clamp(0.0, 1.0)
    }

    /// Set tokens consumed
    pub fn with_tokens(mut self, tokens: usize) -> Self {
        self.tokens_consumed = tokens;
        self
    }

    /// Add a child step
    pub fn add_child(&mut self, child_index: usize) {
        if !self.child_steps.contains(&child_index) {
            self.child_steps.push(child_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_default_confidence() {
        let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
        assert_eq!(step.confidence, 0.5);
    }

    /// @covers: is_confident
    #[test]
    fn test_is_confident_true_above_threshold() {
        let step =
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(0.9);
        assert!(step.is_confident());
    }

    /// @covers: is_leaf
    #[test]
    fn test_is_leaf_true_when_no_children() {
        let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
        assert!(step.is_leaf());
    }

    /// @covers: is_root
    #[test]
    fn test_is_root_true_when_no_parent() {
        let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
        assert!(step.is_root());
    }

    /// @covers: with_confidence
    #[test]
    fn test_with_confidence_clamps_above_one() {
        let step =
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(1.5);
        assert_eq!(step.confidence, 1.0);
    }

    /// @covers: with_tokens
    #[test]
    fn test_with_tokens_sets_value() {
        let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_tokens(5);
        assert_eq!(step.tokens_consumed, 5);
    }

    /// @covers: add_child
    #[test]
    fn test_add_child_avoids_duplicates() {
        let mut step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
        step.add_child(1);
        step.add_child(1);
        assert_eq!(step.child_steps.len(), 1);
    }

    /// @covers: clamp_confidence
    #[test]
    fn test_clamp_confidence_clamps_into_unit_range() {
        assert_eq!(ReasoningStep::clamp_confidence(1.5), 1.0);
        assert_eq!(ReasoningStep::clamp_confidence(-0.5), 0.0);
    }
}
