use std::sync::Arc;

use crate::api::Agent;

/// Response for [`AgentManager::load_agent`](crate::api::traits::AgentManager::load_agent).
pub struct AgentLoadResponse {
    /// The loaded agent.
    pub agent: Arc<dyn Agent>,
}
