use crate::api::types::{AgentLifecycleError, AgentState};

/// LLM agent lifecycle management.
#[async_trait::async_trait]
pub trait AgentLifecycle: Send + Sync {
    /// Current state of the agent.
    fn current_state(&self) -> AgentState;

    /// Transition the agent to a new state.
    async fn transition_to(&self, target: AgentState) -> Result<(), AgentLifecycleError>;

    /// Check whether the agent is currently in the given state.
    fn is_in(&self, state: AgentState) -> bool {
        self.current_state() == state
    }

    /// Pause reasoning (for checkpointing, input waiting).
    async fn pause(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Paused).await
    }

    /// Resume reasoning from the paused state.
    async fn resume(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Running).await
    }

    /// Abort agent execution (return to Idle).
    async fn abort(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Idle).await
    }
}
