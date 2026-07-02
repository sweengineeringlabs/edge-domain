use crate::api::reasoning::types::ThinkingProcess;

/// Request for [`Reasoning::next_step`](crate::api::reasoning::traits::Reasoning::next_step).
#[derive(Debug, Clone, Copy)]
pub struct NextStepRequest<'a> {
    /// In-progress reasoning process.
    pub process: &'a ThinkingProcess,
}
