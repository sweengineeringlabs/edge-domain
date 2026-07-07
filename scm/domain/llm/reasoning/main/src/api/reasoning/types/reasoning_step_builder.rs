//! `ReasoningStepBuilder` — fluent builder for [`ReasoningStep`].

/// Fluent builder for [`ReasoningStep`](crate::api::reasoning::types::ReasoningStep).
///
/// Orphan-type note: exposes its behavior via inherent builder methods, not by implementing
/// a trait, so `no_orphan_types` flags it as unreferenced — accepted tradeoff, same rationale
/// as `StdReasoningFactory`.
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
