use super::AgentState;

#[derive(Debug, thiserror::Error)]
pub enum AgentLifecycleError {
    #[error("Invalid state transition: {from:?} → {to:?}")]
    InvalidTransition { from: AgentState, to: AgentState },

    #[error("Agent is not in expected state: {0:?}")]
    UnexpectedState(AgentState),

    #[error("State transition timeout")]
    TransitionTimeout,

    #[error("Cannot complete: {0}")]
    CompletionFailed(String),
}
