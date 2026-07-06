/// Request for [`Validator::validate_agent_id`](crate::api::traits::Validator::validate_agent_id).
#[derive(Debug, Clone, Copy)]
pub struct AgentIdValidationRequest<'a> {
    /// The agent identifier to validate (e.g., "chief_engineer").
    pub agent_id: &'a str,
}
