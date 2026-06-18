//! No-op [`AgentRegistry`] implementation for testing the contract.

use crate::api::NoopAgentRegistry;
use crate::api::{Agent, AgentError, AgentMetadata, AgentRegistry};
use edge_domain_registry::Registry;
use std::sync::Arc;

impl Registry for NoopAgentRegistry {
    type Value = dyn Agent;

    fn register(&self, id: &str, entry: Arc<Self::Value>) {
        // No-op registry: the entry is intentionally discarded.
        let _ = (id, entry);
    }

    fn try_register(
        &self,
        _id: &str,
        _entry: Arc<Self::Value>,
    ) -> Result<(), edge_domain_registry::RegistryError> {
        Ok(())
    }

    fn deregister(&self, _id: &str) -> bool {
        false
    }

    fn get(&self, _id: &str) -> Option<Arc<Self::Value>> {
        None
    }

    fn list_ids(&self) -> Vec<String> {
        vec![]
    }

    fn len(&self) -> usize {
        0
    }
}

impl AgentRegistry for NoopAgentRegistry {
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError> {
        Err(AgentError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_agent_registry_happy_list_ids_returns_empty() {
        assert_eq!(NoopAgentRegistry.list_ids().len(), 0);
    }

    #[test]
    fn test_noop_agent_registry_happy_len_returns_zero() {
        assert_eq!(NoopAgentRegistry.len(), 0);
    }

    #[test]
    fn test_noop_agent_registry_error_metadata_returns_not_found() {
        let result = NoopAgentRegistry.metadata("any");
        assert!(matches!(result, Err(AgentError::NotFound(_))));
    }

    #[test]
    fn test_noop_agent_registry_edge_get_returns_none() {
        assert!(NoopAgentRegistry.get("any").is_none());
    }
}
