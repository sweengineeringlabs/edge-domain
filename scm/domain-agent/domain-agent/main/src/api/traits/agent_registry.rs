//! AgentRegistry trait — agent discovery and metadata.

use crate::types::AgentMetadata;
use crate::AgentError;

/// AgentRegistry is the contract for agent discovery and metadata retrieval.
///
/// Clients use this to enumerate available agents and query their capabilities.
pub trait AgentRegistry: Send + Sync {
    /// List all available agents.
    fn list_agents(&self) -> Result<Vec<AgentMetadata>, AgentError>;

    /// Get metadata for a specific agent by ID.
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError>;
}
