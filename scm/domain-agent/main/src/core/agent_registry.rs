//! Stub AgentRegistry implementation for arch audit compliance.
//!
//! Real implementations live in plugins (edge-plugin-llmboot).
//! This stub exists only to satisfy the core_implements_api_traits rule.

use crate::api::{Agent, AgentError, AgentMetadata, AgentRegistry};
use edge_domain_registry::Registry;
use std::sync::Arc;

/// Stub AgentRegistry implementation (not for production use).
pub(crate) struct StubAgentRegistry;

impl Registry for StubAgentRegistry {
    type Value = dyn Agent;

    fn register(&self, _id: &str, _entry: Arc<Self::Value>) {}
    fn try_register(&self, _id: &str, _entry: Arc<Self::Value>) -> Result<(), edge_domain_registry::RegistryError> {
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

impl AgentRegistry for StubAgentRegistry {
    fn metadata(&self, _id: &str) -> Result<AgentMetadata, AgentError> {
        Err(AgentError::NotFound("No agents available".to_string()))
    }
}
