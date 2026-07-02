/// Request for [`AgentManager::load_agent`](crate::api::traits::AgentManager::load_agent).
#[derive(Debug, Clone, Copy)]
pub struct AgentLoadRequest<'a> {
    /// Specification (e.g., path to YAML file) describing the agent to load.
    pub spec: &'a str,
}
