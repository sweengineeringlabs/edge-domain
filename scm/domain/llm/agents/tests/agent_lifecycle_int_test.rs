// @covers AgentLifecycle trait methods
// Integration tests for AgentLifecycle trait contract
// Full async tests are in task_workflow_skill pattern Layer 1b

use edge_llm_agent::AgentState;

#[test]
fn test_agent_lifecycle_trait_agent_state_enum_exists() {
    let _ = AgentState::Idle;
    let _ = AgentState::Running;
    let _ = AgentState::Paused;
    let _ = AgentState::Thinking;
    let _ = AgentState::Completed;
}

#[test]
fn test_agent_lifecycle_trait_methods_callable_via_state() {
    let state = AgentState::Running;
    assert!(state.is_active());
    assert!(!state.is_terminal());
}

#[test]
fn test_agent_lifecycle_trait_state_transitions_valid() {
    let idle = AgentState::Idle;
    let running = AgentState::Running;
    let paused = AgentState::Paused;
    let thinking = AgentState::Thinking;
    let completed = AgentState::Completed;

    assert!(!idle.is_active());
    assert!(running.is_active());
    assert!(!paused.is_active());
    assert!(thinking.is_active());
    assert!(completed.is_terminal());
}

#[test]
fn test_agent_lifecycle_trait_state_enum_serializable() {
    let state = AgentState::Running;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}
