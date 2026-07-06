use std::sync::Arc;

use crate::api::Agent;

/// Response for [`AgentManager::agent`](crate::api::traits::AgentManager::agent).
pub struct AgentLookupResponse {
    /// The resolved agent.
    pub agent: Arc<dyn Agent>,
}
