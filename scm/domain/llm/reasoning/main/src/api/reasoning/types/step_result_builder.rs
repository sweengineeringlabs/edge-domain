//! `StepResultBuilder` — fluent builder for [`StepResult`].

/// Fluent builder for [`StepResult`](crate::api::reasoning::types::StepResult).
///
/// Orphan-type note: exposes its behavior via inherent builder methods, not by implementing
/// a trait, so `no_orphan_types` flags it as unreferenced — accepted tradeoff, same rationale
/// as `StdReasoningFactory`.
#[derive(Clone, Debug)]
pub struct StepResultBuilder {
    pub(crate) success: bool,
    pub(crate) output: String,
    pub(crate) error: Option<String>,
    pub(crate) duration_ms: u64,
    pub(crate) should_continue: bool,
    pub(crate) next_action: Option<String>,
}
