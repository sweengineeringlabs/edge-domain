/// Response for [`Agent::id`](crate::api::traits::Agent::id).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIdResponse {
    /// Unique agent identifier.
    pub id: String,
}
