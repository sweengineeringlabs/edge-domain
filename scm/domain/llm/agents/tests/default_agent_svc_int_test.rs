//! Integration tests for the `DEFAULT_AGENT_SVC` service constant.

use edge_llm_agent::DEFAULT_AGENT_SVC;

/// @covers: DEFAULT_AGENT_SVC
#[test]
fn test_default_agent_svc_happy_constant_equals_default_agent() {
    assert_eq!(DEFAULT_AGENT_SVC, "default_agent");
}

/// @covers: DEFAULT_AGENT_SVC
#[test]
fn test_default_agent_svc_error_constant_not_empty() {
    assert!(!DEFAULT_AGENT_SVC.is_empty());
}

/// @covers: DEFAULT_AGENT_SVC
#[test]
fn test_default_agent_svc_edge_constant_is_valid_identifier() {
    assert!(DEFAULT_AGENT_SVC
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_'));
}
