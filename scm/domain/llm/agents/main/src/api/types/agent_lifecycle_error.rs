use super::AgentState;

/// Errors that occur during agent lifecycle state transitions.
#[derive(Debug, thiserror::Error)]
pub enum AgentLifecycleError {
    /// A state transition that is not permitted was attempted.
    #[error("Invalid state transition: {from:?} → {to:?}")]
    InvalidTransition {
        /// The state being transitioned from.
        from: AgentState,
        /// The state being transitioned to.
        to: AgentState,
    },

    /// The agent was not in the state expected by the operation.
    #[error("Agent is not in expected state: {0:?}")]
    UnexpectedState(AgentState),

    /// A state transition did not complete within the allotted time.
    #[error("State transition timeout")]
    TransitionTimeout,

    /// The agent could not be completed.
    #[error("Cannot complete: {0}")]
    CompletionFailed(String),
}
