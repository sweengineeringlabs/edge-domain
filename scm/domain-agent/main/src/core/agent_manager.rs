//! Stub AgentManager implementation for arch audit compliance.
//!
//! Real implementations live in plugins (edge-plugin-llmboot).
//! This stub exists only to satisfy the core_implements_api_traits rule.

use crate::api::{Agent, AgentError, AgentManager};
use std::sync::Arc;

/// Stub AgentManager implementation (not for production use).
pub(crate) struct StubAgentManager;

#[async_trait::async_trait]
impl AgentManager for StubAgentManager {
    async fn load_agent(&self, _spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::InvalidSpec(
            "Stub managers do not load agents".to_string(),
        ))
    }

    fn agent(&self, _id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::NotFound("No agents available".to_string()))
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        Ok(vec![])
    }
}
