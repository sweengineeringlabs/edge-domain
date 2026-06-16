//! Integration tests for service constants and saf/ re-exports.

#[test]
fn test_svc_agent_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_SVC, "agent");
}

#[test]
fn test_svc_agent_manager_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_MANAGER_SVC, "agent_manager");
}

#[test]
fn test_svc_agent_registry_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_REGISTRY_SVC, "agent_registry");
}

#[test]
fn test_svc_agent_svc_error_invalid_name() {
    assert_ne!(edge_domain_agent::AGENT_SVC, "invalid");
}

#[test]
fn test_svc_agent_manager_svc_edge_non_empty_string() {
    assert!(!edge_domain_agent::AGENT_MANAGER_SVC.is_empty());
}
