//! Integration tests for AgentError type re-export via agent_error_svc.rs.

use edge_domain_agent::AgentError;

/// @covers: AgentError type re-export — NotFound variant
#[test]
fn test_svc_agent_error_happy_not_found_variant() {
    let err = AgentError::NotFound("missing_agent".to_string());
    assert_eq!(err.to_string(), "Agent 'missing_agent' not found");
}

/// @covers: AgentError type re-export — SkillNotFound variant
#[test]
fn test_svc_agent_error_happy_skill_not_found_variant() {
    let err = AgentError::SkillNotFound("missing_skill".to_string());
    assert_eq!(err.to_string(), "Skill 'missing_skill' not available on agent");
}

/// @covers: AgentError type re-export — InvalidSpec variant
#[test]
fn test_svc_agent_error_error_invalid_spec_variant() {
    let err = AgentError::InvalidSpec("bad spec".to_string());
    let error_msg = err.to_string();
    assert!(error_msg.contains("Invalid agent specification"));
    assert!(error_msg.contains("bad spec"));
}

/// @covers: AgentError type re-export — ExecutionFailed variant
#[test]
fn test_svc_agent_error_error_execution_failed_variant() {
    let err = AgentError::ExecutionFailed("timeout".to_string());
    let error_msg = err.to_string();
    assert!(error_msg.contains("Skill execution failed"));
    assert!(error_msg.contains("timeout"));
}

/// @covers: AgentError type re-export — InvalidState variant
#[test]
fn test_svc_agent_error_edge_invalid_state_variant() {
    let err = AgentError::InvalidState("not ready".to_string());
    let error_msg = err.to_string();
    assert!(error_msg.contains("not in a valid state"));
    assert!(error_msg.contains("not ready"));
}

/// @covers: AgentError type re-export — Debug trait
#[test]
fn test_svc_agent_error_happy_debug_format_available() {
    let err = AgentError::NotFound("test".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("NotFound"));
}

/// @covers: AgentError type re-export — Error trait implementation
#[test]
fn test_svc_agent_error_happy_implements_error_trait() {
    let err: Box<dyn std::error::Error> = Box::new(AgentError::NotFound("test".to_string()));
    assert!(!err.to_string().is_empty());
}

/// @covers: AgentError type re-export — pattern matching
#[test]
fn test_svc_agent_error_happy_can_pattern_match_not_found() {
    let err = AgentError::NotFound("agent1".to_string());
    match err {
        AgentError::NotFound(id) => assert_eq!(id, "agent1"),
        _ => panic!("Expected NotFound variant"),
    }
}

/// @covers: AgentError type re-export — pattern matching
#[test]
fn test_svc_agent_error_happy_can_pattern_match_skill_not_found() {
    let err = AgentError::SkillNotFound("skill1".to_string());
    match err {
        AgentError::SkillNotFound(name) => assert_eq!(name, "skill1"),
        _ => panic!("Expected SkillNotFound variant"),
    }
}

/// @covers: AgentError type re-export — pattern matching
#[test]
fn test_svc_agent_error_happy_can_pattern_match_execution_failed() {
    let err = AgentError::ExecutionFailed("error details".to_string());
    match err {
        AgentError::ExecutionFailed(details) => assert_eq!(details, "error details"),
        _ => panic!("Expected ExecutionFailed variant"),
    }
}

/// @covers: AgentError type re-export — all variants distinguishable
#[test]
fn test_svc_agent_error_happy_all_variants_are_distinct() {
    let not_found = AgentError::NotFound("test".to_string());
    let skill_not_found = AgentError::SkillNotFound("test".to_string());
    let invalid_spec = AgentError::InvalidSpec("test".to_string());
    let execution_failed = AgentError::ExecutionFailed("test".to_string());
    let invalid_state = AgentError::InvalidState("test".to_string());

    // All should have different display messages (except they share "test")
    let messages = vec![
        not_found.to_string(),
        skill_not_found.to_string(),
        invalid_spec.to_string(),
        execution_failed.to_string(),
        invalid_state.to_string(),
    ];

    for (i, msg1) in messages.iter().enumerate() {
        for (j, msg2) in messages.iter().enumerate() {
            if i != j {
                assert_ne!(msg1, msg2, "Error variants should have different messages");
            }
        }
    }
}

/// @covers: AgentError type re-export — edge case empty string
#[test]
fn test_svc_agent_error_edge_empty_string_message() {
    let err = AgentError::NotFound(String::new());
    assert_eq!(err.to_string(), "Agent '' not found");
}

/// @covers: AgentError type re-export — error message formatting
#[test]
fn test_svc_agent_error_happy_error_messages_are_descriptive() {
    let err1 = AgentError::NotFound("my_agent".to_string());
    let err2 = AgentError::SkillNotFound("my_skill".to_string());

    assert!(err1.to_string().contains("my_agent"));
    assert!(err2.to_string().contains("my_skill"));
}
