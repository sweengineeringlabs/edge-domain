/// Request for [`AgentManager::agent`](crate::api::traits::AgentManager::agent).
#[derive(Debug, Clone, Copy)]
pub struct AgentLookupRequest<'a> {
    /// ID of the loaded agent to look up.
    pub id: &'a str,
}
