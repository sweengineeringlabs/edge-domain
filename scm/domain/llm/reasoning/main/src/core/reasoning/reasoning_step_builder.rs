//! Constructors and builder methods for [`ReasoningStepBuilder`].

use crate::api::{ReasoningStep, ReasoningStepBuilder};

impl ReasoningStepBuilder {
    /// Start a new builder for the step at `index`.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            content: String::new(),
            step_type: String::new(),
            confidence: 0.5,
            tokens_consumed: 0,
            parent_step: None,
            child_steps: vec![],
        }
    }

    /// Set the reasoning content.
    pub fn content(mut self, value: String) -> Self {
        self.content = value;
        self
    }

    /// Set the step type.
    pub fn step_type(mut self, value: String) -> Self {
        self.step_type = value;
        self
    }

    /// Set the confidence (clamped to `0.0..=1.0`).
    pub fn confidence(mut self, value: f32) -> Self {
        self.confidence = value.clamp(0.0, 1.0);
        self
    }

    /// Set the tokens consumed.
    pub fn tokens_consumed(mut self, value: usize) -> Self {
        self.tokens_consumed = value;
        self
    }

    /// Set the parent step index.
    pub fn parent_step(mut self, value: usize) -> Self {
        self.parent_step = Some(value);
        self
    }

    /// Add a child step index.
    pub fn child_step(mut self, value: usize) -> Self {
        if !self.child_steps.contains(&value) {
            self.child_steps.push(value);
        }
        self
    }

    /// Build the [`ReasoningStep`].
    pub fn build(self) -> ReasoningStep {
        let step = ReasoningStep::new(self.index, self.content, self.step_type)
            .with_confidence(self.confidence)
            .with_tokens(self.tokens_consumed);
        Self::apply_lineage(step, self.parent_step, self.child_steps)
    }

    /// Apply the accumulated parent/child lineage onto an already-built step.
    fn apply_lineage(
        mut step: ReasoningStep,
        parent_step: Option<usize>,
        child_steps: Vec<usize>,
    ) -> ReasoningStep {
        step.parent_step = parent_step;
        step.child_steps = child_steps;
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_content() {
        let builder = ReasoningStepBuilder::new(0);
        assert_eq!(builder.content, "");
    }

    /// @covers: content
    #[test]
    fn test_content_sets_value() {
        let builder = ReasoningStepBuilder::new(0).content("x".to_string());
        assert_eq!(builder.content, "x");
    }

    /// @covers: build
    #[test]
    fn test_build_produces_reasoning_step() {
        let step = ReasoningStepBuilder::new(0)
            .content("x".to_string())
            .parent_step(3)
            .build();
        assert_eq!(step.parent_step, Some(3));
    }

    /// @covers: apply_lineage
    #[test]
    fn test_apply_lineage_sets_parent_and_children() {
        let step = ReasoningStep::new(0, "x".to_string(), "analysis".to_string());
        let step = ReasoningStepBuilder::apply_lineage(step, Some(2), vec![5, 6]);
        assert_eq!(step.parent_step, Some(2));
        assert_eq!(step.child_steps, vec![5, 6]);
    }

    /// @covers: step_type
    #[test]
    fn test_step_type_sets_value() {
        let step = ReasoningStepBuilder::new(0)
            .step_type("hypothesis".to_string())
            .build();
        assert_eq!(step.step_type, "hypothesis");
    }

    /// @covers: confidence
    #[test]
    fn test_confidence_clamps_above_one() {
        let step = ReasoningStepBuilder::new(0).confidence(1.5).build();
        assert_eq!(step.confidence, 1.0);
    }

    /// @covers: tokens_consumed
    #[test]
    fn test_tokens_consumed_sets_value() {
        let step = ReasoningStepBuilder::new(0).tokens_consumed(7).build();
        assert_eq!(step.tokens_consumed, 7);
    }

    /// @covers: parent_step
    #[test]
    fn test_parent_step_sets_value() {
        let step = ReasoningStepBuilder::new(0).parent_step(4).build();
        assert_eq!(step.parent_step, Some(4));
    }

    /// @covers: child_step
    #[test]
    fn test_child_step_avoids_duplicates() {
        let step = ReasoningStepBuilder::new(0)
            .child_step(1)
            .child_step(1)
            .build();
        assert_eq!(step.child_steps, vec![1]);
    }
}
