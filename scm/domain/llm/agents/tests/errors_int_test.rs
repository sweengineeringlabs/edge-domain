//! Integration tests for AgentError type.

use edge_domain_agent::AgentError;

#[test]
fn test_type_agent_error_happy_not_found_variant() {
    let err = AgentError::NotFound("test_agent".to_string());
    assert_eq!(err.to_string(), "Agent 'test_agent' not found");
}

#[test]
fn test_type_agent_error_happy_skill_not_found_variant() {
    let err = AgentError::SkillNotFound("test_skill".to_string());
    assert_eq!(err.to_string(), "Skill 'test_skill' not available on agent");
}

#[test]
fn test_type_agent_error_error_invalid_spec_variant() {
    let err = AgentError::InvalidSpec("bad config".to_string());
    assert!(err.to_string().contains("Invalid agent specification"));
}

#[test]
fn test_type_agent_error_error_execution_failed_variant() {
    let err = AgentError::ExecutionFailed("timeout".to_string());
    assert!(err.to_string().contains("Skill execution failed"));
}

#[test]
fn test_type_agent_error_edge_invalid_state_variant() {
    let err = AgentError::InvalidState("not ready".to_string());
    assert!(err.to_string().contains("not in a valid state"));
}
