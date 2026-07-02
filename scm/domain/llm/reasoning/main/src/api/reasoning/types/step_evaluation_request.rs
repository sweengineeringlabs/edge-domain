use crate::api::reasoning::types::ReasoningStep;

/// Request for [`Reasoning::evaluate_step`](crate::api::reasoning::traits::Reasoning::evaluate_step).
#[derive(Debug, Clone, Copy)]
pub struct StepEvaluationRequest<'a> {
    /// Completed reasoning step to evaluate.
    pub step: &'a ReasoningStep,
}
