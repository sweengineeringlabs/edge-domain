/// Response for [`AgentManager::list_agent_ids`](crate::api::traits::AgentManager::list_agent_ids).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAgentIdsResponse {
    /// All loaded agent IDs.
    pub ids: Vec<String>,
}
