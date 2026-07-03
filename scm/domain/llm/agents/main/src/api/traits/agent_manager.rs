//! AgentManager trait — service for loading and accessing agents.

use crate::api::types::{
    AgentCreationRequest, AgentCreationResponse, AgentHandlerRequest, AgentHandlerResponse,
    AgentLoadRequest, AgentLoadResponse, AgentLookupRequest, AgentLookupResponse,
    AgentMetadataBuilderRequest, AgentMetadataBuilderResponse, ConversationLoopRequest,
    ConversationLoopResponse, ListAgentIdsRequest, ListAgentIdsResponse,
    SkillMetadataBuilderRequest, SkillMetadataBuilderResponse,
};
use crate::api::AgentError;

/// AgentManager is the service that loads and provides access to agents.
///
/// Implementations parse agent specifications (e.g., YAML) and instantiate Agent instances.
#[async_trait::async_trait]
pub trait AgentManager: Send + Sync {
    /// Load an agent from a specification (e.g., path to YAML file).
    async fn load_agent(&self, req: AgentLoadRequest<'_>) -> Result<AgentLoadResponse, AgentError>;

    /// Get a loaded agent by ID.
    fn agent(&self, req: AgentLookupRequest<'_>) -> Result<AgentLookupResponse, AgentError>;

    /// List all loaded agent IDs.
    fn list_agent_ids(&self, req: ListAgentIdsRequest) -> Result<ListAgentIdsResponse, AgentError>;

    /// Return a dispatchable handler that routes requests to the named skill.
    fn agent_handler(
        &self,
        req: AgentHandlerRequest<'_>,
    ) -> Result<AgentHandlerResponse, AgentError>;

    /// Construct a concrete [`Agent`](crate::api::traits::Agent) backed by the given provider and skill registry.
    fn default_agent(
        &self,
        req: AgentCreationRequest<'_>,
    ) -> Result<AgentCreationResponse, AgentError>;

    /// Create a builder for constructing AgentMetadata.
    fn agent_metadata_builder(
        &self,
        _req: AgentMetadataBuilderRequest,
    ) -> Result<AgentMetadataBuilderResponse, AgentError> {
        Ok(AgentMetadataBuilderResponse {
            builder: Box::new(crate::api::types::AgentMetadataBuilder::new()),
        })
    }

    /// Create a builder for constructing SkillMetadata.
    fn skill_metadata_builder(
        &self,
        _req: SkillMetadataBuilderRequest,
    ) -> Result<SkillMetadataBuilderResponse, AgentError> {
        Ok(SkillMetadataBuilderResponse {
            builder: Box::new(crate::api::types::SkillMetadataBuilder::new()),
        })
    }

    /// Build a [`ConversationLoop`](crate::api::traits::ConversationLoop) that drives
    /// `req.agent` through a bounded multi-turn conversation.
    fn conversation_loop(
        &self,
        req: ConversationLoopRequest,
    ) -> Result<ConversationLoopResponse, AgentError> {
        Ok(ConversationLoopResponse {
            conversation_loop: Box::new(crate::api::types::BoundedConversationLoop {
                agent: req.agent,
            }),
        })
    }
}
