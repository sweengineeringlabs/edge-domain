use std::sync::Arc;

use crate::api::Agent;

/// Response for [`AgentManager::default_agent`](crate::api::traits::AgentManager::default_agent).
pub struct AgentCreationResponse {
    /// The constructed agent.
    pub agent: Arc<dyn Agent>,
}
