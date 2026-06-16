use edge_llm_agent::AgentState;

#[test]
fn test_agent_state_idle_not_terminal() {
    assert!(!AgentState::Idle.is_terminal());
}

#[test]
fn test_agent_state_running_not_terminal() {
    assert!(!AgentState::Running.is_terminal());
}

#[test]
fn test_agent_state_paused_not_terminal() {
    assert!(!AgentState::Paused.is_terminal());
}

#[test]
fn test_agent_state_thinking_not_terminal() {
    assert!(!AgentState::Thinking.is_terminal());
}

#[test]
fn test_agent_state_completed_is_terminal() {
    assert!(AgentState::Completed.is_terminal());
}

#[test]
fn test_agent_state_idle_not_active() {
    assert!(!AgentState::Idle.is_active());
}

#[test]
fn test_agent_state_running_is_active() {
    assert!(AgentState::Running.is_active());
}

#[test]
fn test_agent_state_paused_not_active() {
    assert!(!AgentState::Paused.is_active());
}

#[test]
fn test_agent_state_thinking_is_active() {
    assert!(AgentState::Thinking.is_active());
}

#[test]
fn test_agent_state_completed_not_active() {
    assert!(!AgentState::Completed.is_active());
}

#[test]
fn test_agent_state_all_variants_constructible() {
    let _idle = AgentState::Idle;
    let _running = AgentState::Running;
    let _paused = AgentState::Paused;
    let _thinking = AgentState::Thinking;
    let _completed = AgentState::Completed;
}

#[test]
fn test_agent_state_equality() {
    assert_eq!(AgentState::Idle, AgentState::Idle);
    assert_ne!(AgentState::Idle, AgentState::Running);
    assert_eq!(AgentState::Completed, AgentState::Completed);
}

#[test]
fn test_agent_state_clone() {
    let state = AgentState::Running;
    let cloned = state.clone();
    assert_eq!(state, cloned);
}

#[test]
fn test_agent_state_copy() {
    let state = AgentState::Paused;
    let copied = state;
    assert_eq!(state, copied);
}

#[test]
fn test_agent_state_debug_format() {
    assert_eq!(format!("{:?}", AgentState::Idle), "Idle");
    assert_eq!(format!("{:?}", AgentState::Running), "Running");
    assert_eq!(format!("{:?}", AgentState::Completed), "Completed");
}

#[test]
fn test_agent_state_serialization_deserialize_idle() {
    let state = AgentState::Idle;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}

#[test]
fn test_agent_state_serialization_deserialize_running() {
    let state = AgentState::Running;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}

#[test]
fn test_agent_state_serialization_deserialize_paused() {
    let state = AgentState::Paused;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}

#[test]
fn test_agent_state_serialization_deserialize_thinking() {
    let state = AgentState::Thinking;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}

#[test]
fn test_agent_state_serialization_deserialize_completed() {
    let state = AgentState::Completed;
    let json = serde_json::to_string(&state).expect("serialize");
    let deserialized: AgentState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(state, deserialized);
}

#[test]
fn test_agent_state_json_format_idle() {
    let state = AgentState::Idle;
    let json = serde_json::to_string(&state).expect("serialize");
    assert_eq!(json, "\"Idle\"");
}

#[test]
fn test_agent_state_json_format_completed() {
    let state = AgentState::Completed;
    let json = serde_json::to_string(&state).expect("serialize");
    assert_eq!(json, "\"Completed\"");
}

#[test]
fn test_agent_state_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let state1 = AgentState::Running;
    let state2 = AgentState::Running;

    let mut hasher1 = DefaultHasher::new();
    state1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    state2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_agent_state_different_hashes() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let state1 = AgentState::Running;
    let state2 = AgentState::Paused;

    let mut hasher1 = DefaultHasher::new();
    state1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    state2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_ne!(hash1, hash2);
}
