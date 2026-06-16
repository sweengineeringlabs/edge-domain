//! Integration tests for AGENT_REGISTRY_SVC constant.

#[test]
fn agent_registry_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_REGISTRY_SVC, "agent_registry");
}
