//! Integration tests for AGENT_SVC constant.

#[test]
fn agent_svc_happy_constant_defined() {
    assert_eq!(edge_domain_agent::AGENT_SVC, "agent");
}
