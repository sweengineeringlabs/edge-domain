/// Request for [`ExecutionModel::execute_step`](crate::api::provider::traits::ExecutionModel::execute_step).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepExecutionRequest<'a> {
    /// Identifier of the requesting agent.
    pub agent_id: &'a str,
    /// Goal to reason toward.
    pub goal: &'a str,
    /// Prior context available to the step.
    pub context: &'a str,
    /// Tools available for this step.
    pub available_tools: Vec<String>,
}
