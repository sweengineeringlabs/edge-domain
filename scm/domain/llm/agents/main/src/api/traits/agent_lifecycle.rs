use crate::api::types::{
    AbortRequest, AgentLifecycleError, AgentState, CurrentStateRequest, CurrentStateResponse,
    PauseRequest, ResumeRequest, StateCheckRequest, StateCheckResponse, TransitionRequest,
};

/// LLM agent lifecycle management.
#[async_trait::async_trait]
pub trait AgentLifecycle: Send + Sync {
    /// Current state of the agent.
    fn current_state(
        &self,
        req: CurrentStateRequest,
    ) -> Result<CurrentStateResponse, AgentLifecycleError>;

    /// Transition the agent to a new state.
    async fn transition_to(&self, req: TransitionRequest) -> Result<(), AgentLifecycleError>;

    /// Check whether the agent is currently in the given state.
    fn is_in(&self, req: StateCheckRequest) -> Result<StateCheckResponse, AgentLifecycleError> {
        let matches = self.current_state(CurrentStateRequest)?.state == req.state;
        Ok(StateCheckResponse { matches })
    }

    /// Pause reasoning (for checkpointing, input waiting).
    async fn pause(&self, _req: PauseRequest) -> Result<(), AgentLifecycleError> {
        self.transition_to(TransitionRequest {
            target: AgentState::Paused,
        })
        .await
    }

    /// Resume reasoning from the paused state.
    async fn resume(&self, _req: ResumeRequest) -> Result<(), AgentLifecycleError> {
        self.transition_to(TransitionRequest {
            target: AgentState::Running,
        })
        .await
    }

    /// Abort agent execution (return to Idle).
    async fn abort(&self, _req: AbortRequest) -> Result<(), AgentLifecycleError> {
        self.transition_to(TransitionRequest {
            target: AgentState::Idle,
        })
        .await
    }
}
