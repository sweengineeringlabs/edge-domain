use crate::api::provider::types::ExecutionStepResult;

/// Response for [`ExecutionModel::execute_step`](crate::api::provider::traits::ExecutionModel::execute_step).
#[derive(Debug, Clone)]
pub struct StepExecutionResponse {
    /// Result of the executed step.
    pub result: Box<ExecutionStepResult>,
}
