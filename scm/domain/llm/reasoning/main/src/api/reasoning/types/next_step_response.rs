use crate::api::reasoning::types::ReasoningStep;

/// Response for [`Reasoning::next_step`](crate::api::reasoning::traits::Reasoning::next_step).
#[derive(Debug, Clone)]
pub struct NextStepResponse {
    /// The proposed next reasoning step.
    pub step: Box<ReasoningStep>,
}
