/// Request for [`AgentRegistry::metadata`](crate::api::traits::AgentRegistry::metadata).
#[derive(Debug, Clone, Copy)]
pub struct AgentMetadataLookupRequest<'a> {
    /// ID of the agent to look up metadata for.
    pub id: &'a str,
}
