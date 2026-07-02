/// Response for [`Agent::name`](crate::api::traits::Agent::name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentNameResponse {
    /// Human-readable agent name.
    pub name: String,
}
