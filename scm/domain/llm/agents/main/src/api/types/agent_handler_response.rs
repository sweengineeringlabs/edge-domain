use edge_domain_handler::Handler;

/// Response for [`AgentManager::agent_handler`](crate::api::traits::AgentManager::agent_handler).
pub struct AgentHandlerResponse {
    /// Dispatchable handler that routes requests to the named skill.
    pub handler: Box<dyn Handler<Request = String, Response = String>>,
}
