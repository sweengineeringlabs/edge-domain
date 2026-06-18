//! No-op [`AgentManager`] implementation for testing the contract.

use crate::api::NoopAgentManager;
use crate::api::{Agent, AgentError, AgentManager};
use std::sync::Arc;

#[async_trait::async_trait]
impl AgentManager for NoopAgentManager {
    async fn load_agent(&self, _spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::InvalidSpec("No-op manager".to_string()))
    }

    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::NotFound(id.to_string()))
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_agent_manager_happy_list_agent_ids_returns_empty() {
        let ids = NoopAgentManager.list_agent_ids().unwrap_or_default();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_noop_agent_manager_error_agent_returns_not_found() {
        let result = NoopAgentManager.agent("any");
        assert!(matches!(result, Err(AgentError::NotFound(_))));
    }

    #[test]
    fn test_noop_agent_manager_error_load_agent_returns_invalid_spec() {
        let result = futures::executor::block_on(NoopAgentManager.load_agent("spec"));
        assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
    }
}
