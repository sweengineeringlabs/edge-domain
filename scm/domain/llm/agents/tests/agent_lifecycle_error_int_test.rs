#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `AgentLifecycleError` type.

use edge_llm_agent::{AgentLifecycleError, AgentState};

#[test]
fn test_agent_lifecycle_error_invalid_transition_display() {
    let err = AgentLifecycleError::InvalidTransition {
        from: AgentState::Completed,
        to: AgentState::Running,
    };
    let msg = err.to_string();
    assert!(msg.contains("Invalid state transition"));
}

#[test]
fn test_agent_lifecycle_error_unexpected_state_display() {
    let err = AgentLifecycleError::UnexpectedState(AgentState::Paused);
    assert!(err.to_string().contains("not in expected state"));
}

#[test]
fn test_agent_lifecycle_error_completion_failed_carries_reason() {
    let err = AgentLifecycleError::CompletionFailed("disk full".to_string());
    assert!(err.to_string().contains("disk full"));
}
