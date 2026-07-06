/// Response for [`Agent::description`](crate::api::traits::Agent::description).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDescriptionResponse {
    /// Agent description and purpose.
    pub description: String,
}
