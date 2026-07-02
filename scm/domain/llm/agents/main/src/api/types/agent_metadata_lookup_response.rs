use crate::api::types::AgentMetadata;

/// Response for [`AgentRegistry::metadata`](crate::api::traits::AgentRegistry::metadata).
pub struct AgentMetadataLookupResponse {
    /// Metadata for the resolved agent.
    pub metadata: Box<AgentMetadata>,
}
