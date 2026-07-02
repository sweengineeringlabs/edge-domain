/// Response for [`AgentLifecycle::is_in`](crate::api::traits::AgentLifecycle::is_in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateCheckResponse {
    /// Whether the agent is currently in the queried state.
    pub matches: bool,
}
