use crate::api::types::{AgentState, AgentLifecycleError};

/// LLM agent lifecycle management
pub trait AgentLifecycle: Send + Sync {
    /// Current state of agent
    fn current_state(&self) -> AgentState;

    /// Transition agent to new state
    async fn transition_to(&self, target: AgentState) -> Result<(), AgentLifecycleError>;

    /// Check if agent is in specific state
    fn is_in(&self, state: AgentState) -> bool {
        self.current_state() == state
    }

    /// Pause reasoning (for checkpointing, input waiting)
    async fn pause(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Paused).await
    }

    /// Resume reasoning from paused state
    async fn resume(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Running).await
    }

    /// Abort agent execution (return to Idle)
    async fn abort(&self) -> Result<(), AgentLifecycleError> {
        self.transition_to(AgentState::Idle).await
    }
}
