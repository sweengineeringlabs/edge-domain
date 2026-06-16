use serde::{Deserialize, Serialize};

/// LLM agent lifecycle state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AgentState {
    /// Agent created but not yet started
    Idle,

    /// Agent actively reasoning/executing
    Running,

    /// Agent paused (waiting for input, checkpointing, etc.)
    Paused,

    /// Agent in reasoning phase (thinking deeply, exploring options)
    Thinking,

    /// Agent completed successfully
    Completed,
}

impl AgentState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, AgentState::Completed)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, AgentState::Running | AgentState::Thinking)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_state_idle_not_terminal() {
        assert!(!AgentState::Idle.is_terminal());
    }

    #[test]
    fn test_agent_state_completed_is_terminal() {
        assert!(AgentState::Completed.is_terminal());
    }

    #[test]
    fn test_agent_state_running_is_active() {
        assert!(AgentState::Running.is_active());
    }

    #[test]
    fn test_agent_state_thinking_is_active() {
        assert!(AgentState::Thinking.is_active());
    }

    #[test]
    fn test_agent_state_paused_not_active() {
        assert!(!AgentState::Paused.is_active());
    }
}
