//! AgentManager trait — service for loading and accessing agents.

use std::sync::Arc;

use super::agent::Agent;
use crate::api::types::{AgentMetadataBuilder, SkillMetadataBuilder};
use crate::AgentError;

/// AgentManager is the service that loads and provides access to agents.
///
/// Implementations parse agent specifications (e.g., YAML) and instantiate Agent instances.
#[async_trait::async_trait]
pub trait AgentManager: Send + Sync {
    /// Load an agent from a specification (e.g., path to YAML file).
    async fn load_agent(&self, spec: &str) -> Result<Arc<dyn Agent>, AgentError>;

    /// Get a loaded agent by ID.
    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError>;

    /// List all loaded agent IDs.
    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError>;

    /// Create a builder for constructing AgentMetadata.
    fn agent_metadata_builder(&self) -> AgentMetadataBuilder {
        AgentMetadataBuilder::new()
    }

    /// Create a builder for constructing SkillMetadata.
    fn skill_metadata_builder(&self) -> SkillMetadataBuilder {
        SkillMetadataBuilder::new()
    }
}
