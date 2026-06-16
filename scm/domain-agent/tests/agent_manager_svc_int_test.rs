//! Integration tests for AGENT_MANAGER_SVC constant.

#[test]
fn agent_manager_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_MANAGER_SVC, "agent_manager");
}
