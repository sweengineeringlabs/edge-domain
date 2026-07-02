//! `ReasoningStepBuilder` — fluent builder for [`ReasoningStep`].

/// Fluent builder for [`ReasoningStep`](crate::api::reasoning::types::ReasoningStep).
#[derive(Clone, Debug)]
pub struct ReasoningStepBuilder {
    pub(crate) index: usize,
    pub(crate) content: String,
    pub(crate) step_type: String,
    pub(crate) confidence: f32,
    pub(crate) tokens_consumed: usize,
    pub(crate) parent_step: Option<usize>,
    pub(crate) child_steps: Vec<usize>,
}
