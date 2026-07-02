use crate::api::types::AgentState;

/// Response for [`AgentLifecycle::current_state`](crate::api::traits::AgentLifecycle::current_state).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentStateResponse {
    /// Current state of the agent.
    pub state: AgentState,
}
