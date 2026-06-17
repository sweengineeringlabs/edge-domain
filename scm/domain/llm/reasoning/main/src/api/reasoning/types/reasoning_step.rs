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
        self.confidence = confidence.clamp(0.0, 1.0);
        self
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
