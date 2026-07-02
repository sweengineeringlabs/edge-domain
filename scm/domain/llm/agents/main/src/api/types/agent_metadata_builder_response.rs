use crate::api::types::AgentMetadataBuilder;

/// Response for [`AgentManager::agent_metadata_builder`](crate::api::traits::AgentManager::agent_metadata_builder).
pub struct AgentMetadataBuilderResponse {
    /// A builder for constructing [`AgentMetadata`](crate::api::types::AgentMetadata).
    pub builder: Box<AgentMetadataBuilder>,
}
