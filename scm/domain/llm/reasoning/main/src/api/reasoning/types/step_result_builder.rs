//! `StepResultBuilder` — fluent builder for [`StepResult`].

/// Fluent builder for [`StepResult`](crate::api::reasoning::types::StepResult).
#[derive(Clone, Debug)]
pub struct StepResultBuilder {
    pub(crate) success: bool,
    pub(crate) output: String,
    pub(crate) error: Option<String>,
    pub(crate) duration_ms: u64,
    pub(crate) should_continue: bool,
    pub(crate) next_action: Option<String>,
}
