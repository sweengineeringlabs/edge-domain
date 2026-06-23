//! No-op [`AgentLifecycle`] implementation for testing the contract.

use crate::api::NoopAgentLifecycle;
use crate::api::{AgentLifecycle, AgentLifecycleError, AgentState};

#[async_trait::async_trait]
impl AgentLifecycle for NoopAgentLifecycle {
    fn current_state(&self) -> AgentState {
        AgentState::Idle
    }

    async fn transition_to(&self, target: AgentState) -> Result<(), AgentLifecycleError> {
        // A no-op lifecycle never leaves Idle: any transition away is rejected,
        // while a redundant transition back to Idle is accepted.
        if target == AgentState::Idle {
            Ok(())
        } else {
            Err(AgentLifecycleError::InvalidTransition {
                from: AgentState::Idle,
                to: target,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_noop_agent_lifecycle_current_state_is_idle() {
        assert_eq!(NoopAgentLifecycle.current_state(), AgentState::Idle);
    }

    #[test]
    fn test_noop_agent_lifecycle_transition_to_idle_ok() {
        assert_eq!(block_on(NoopAgentLifecycle.transition_to(AgentState::Idle)), Ok(()));
    }

    #[test]
    fn test_noop_agent_lifecycle_transition_to_running_rejected() {
        let result = block_on(NoopAgentLifecycle.transition_to(AgentState::Running));
        assert!(matches!(
            result,
            Err(AgentLifecycleError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn test_noop_agent_lifecycle_is_in_idle_true() {
        assert!(NoopAgentLifecycle.is_in(AgentState::Idle));
    }
}
