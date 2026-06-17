//! `ReasoningStepBuilder` — fluent builder for [`ReasoningStep`].

use crate::api::reasoning::types::ReasoningStep;

/// Fluent builder for [`ReasoningStep`].
#[derive(Clone, Debug)]
pub struct ReasoningStepBuilder {
    index: usize,
    content: String,
    step_type: String,
    confidence: f32,
    tokens_consumed: usize,
    parent_step: Option<usize>,
    child_steps: Vec<usize>,
}

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
        let mut step = ReasoningStep::new(self.index, self.content, self.step_type)
            .with_confidence(self.confidence)
            .with_tokens(self.tokens_consumed);
        step.parent_step = self.parent_step;
        step.child_steps = self.child_steps;
        step
    }
}
