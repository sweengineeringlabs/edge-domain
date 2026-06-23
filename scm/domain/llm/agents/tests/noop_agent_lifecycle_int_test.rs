#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the no-op `AgentLifecycle` implementation.

use edge_llm_agent::{AgentLifecycle, AgentState, NoopAgentLifecycle};
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
fn test_noop_agent_lifecycle_transition_away_rejected() {
    assert!(block_on(NoopAgentLifecycle.transition_to(AgentState::Running)).is_err());
}
