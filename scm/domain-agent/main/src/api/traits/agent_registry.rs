//! AgentRegistry trait — agent discovery and metadata.

use edge_domain_registry::Registry;

use super::Agent;
use crate::api::types::AgentMetadata;
use crate::api::AgentError;

/// AgentRegistry specializes the generic [`Registry`] for agents.
///
/// Inherits register, deregister, get, list_ids, len, is_empty from Registry.
/// Adds agent-specific metadata retrieval.
pub trait AgentRegistry: Registry<Value = dyn Agent> {
    /// Get metadata for a specific agent by ID.
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError>;
}
