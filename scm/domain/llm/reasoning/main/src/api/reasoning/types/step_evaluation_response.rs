use crate::api::reasoning::types::StepResult;

/// Response for [`Reasoning::evaluate_step`](crate::api::reasoning::traits::Reasoning::evaluate_step).
#[derive(Debug, Clone)]
pub struct StepEvaluationResponse {
    /// Result of evaluating the step.
    pub result: Box<StepResult>,
}
