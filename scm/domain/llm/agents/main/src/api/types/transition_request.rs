use crate::api::types::AgentState;

/// Request for [`AgentLifecycle::transition_to`](crate::api::traits::AgentLifecycle::transition_to).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionRequest {
    /// The state to transition to.
    pub target: AgentState,
}
