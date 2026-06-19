//! AgentManager trait — service for loading and accessing agents.

use std::sync::Arc;

use edge_domain_handler::Handler;
use edge_llm_provider::Provider;

use super::agent::Agent;
use super::skill::Skill;
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

    /// Return a dispatchable handler that routes requests to the named skill.
    fn agent_handler(&self, skill: &str) -> Box<dyn Handler<Request = String, Response = String>>;

    /// Construct a concrete [`Agent`] backed by the given provider and skill registry.
    fn default_agent(
        &self,
        id: &str,
        name: &str,
        description: &str,
        provider: Arc<dyn Provider>,
        skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
    ) -> Arc<dyn Agent>;

    /// Create a builder for constructing AgentMetadata.
    fn agent_metadata_builder(&self) -> AgentMetadataBuilder {
        AgentMetadataBuilder::new()
    }

    /// Create a builder for constructing SkillMetadata.
    fn skill_metadata_builder(&self) -> SkillMetadataBuilder {
        SkillMetadataBuilder::new()
    }
}
