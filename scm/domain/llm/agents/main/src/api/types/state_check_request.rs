use crate::api::types::AgentState;

/// Request for [`AgentLifecycle::is_in`](crate::api::traits::AgentLifecycle::is_in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateCheckRequest {
    /// The state to check membership against.
    pub state: AgentState,
}
