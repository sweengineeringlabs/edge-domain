/// Request for [`AgentManager::agent_handler`](crate::api::traits::AgentManager::agent_handler).
#[derive(Debug, Clone, Copy)]
pub struct AgentHandlerRequest<'a> {
    /// Name of the skill to route requests to.
    pub skill: &'a str,
}
