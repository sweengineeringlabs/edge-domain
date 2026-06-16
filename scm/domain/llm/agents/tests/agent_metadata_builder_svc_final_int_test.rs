//! Tests for AGENT_METADATA_BUILDER_SVC constant.

use edge_llm_agent::AGENT_METADATA_BUILDER_SVC;

#[test]
fn test_agent_metadata_builder_svc_constant_exists() {
    // @covers AGENT_METADATA_BUILDER_SVC constant
    assert!(!AGENT_METADATA_BUILDER_SVC.is_empty());
}

#[test]
fn test_agent_metadata_builder_svc_is_valid_identifier() {
    // @covers AGENT_METADATA_BUILDER_SVC - valid service name
    assert_eq!(AGENT_METADATA_BUILDER_SVC, "agent_metadata_builder");
}

#[test]
fn test_agent_metadata_builder_svc_constant_is_string() {
    // @covers AGENT_METADATA_BUILDER_SVC - type verification
    let svc: &str = AGENT_METADATA_BUILDER_SVC;
    assert!(!svc.is_empty());
}
