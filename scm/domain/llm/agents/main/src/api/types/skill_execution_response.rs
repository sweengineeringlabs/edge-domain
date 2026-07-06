/// Response for [`Agent::execute_skill`](crate::api::traits::Agent::execute_skill).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillExecutionResponse {
    /// Serialized output from the skill execution.
    pub output: String,
}
