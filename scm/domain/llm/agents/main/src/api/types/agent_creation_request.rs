use std::sync::Arc;

use edge_llm_provider::Provider;

use crate::api::Skill;

/// Request for [`AgentManager::default_agent`](crate::api::traits::AgentManager::default_agent).
pub struct AgentCreationRequest<'a> {
    /// Unique agent identifier.
    pub id: &'a str,
    /// Human-readable agent name.
    pub name: &'a str,
    /// Agent description and purpose.
    pub description: &'a str,
    /// The LLM provider this agent delegates completions to.
    pub provider: Arc<dyn Provider>,
    /// Skills the agent exposes.
    pub skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
}
