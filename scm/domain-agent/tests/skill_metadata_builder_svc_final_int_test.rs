//! Tests for SKILL_METADATA_BUILDER_SVC constant.

use edge_domain_agent::SKILL_METADATA_BUILDER_SVC;

#[test]
fn test_skill_metadata_builder_svc_constant_exists() {
    // @covers SKILL_METADATA_BUILDER_SVC constant
    assert!(!SKILL_METADATA_BUILDER_SVC.is_empty());
}

#[test]
fn test_skill_metadata_builder_svc_is_valid_identifier() {
    // @covers SKILL_METADATA_BUILDER_SVC - valid service name
    assert_eq!(SKILL_METADATA_BUILDER_SVC, "skill_metadata_builder");
}

#[test]
fn test_skill_metadata_builder_svc_constant_is_string() {
    // @covers SKILL_METADATA_BUILDER_SVC - type verification
    let svc: &str = SKILL_METADATA_BUILDER_SVC;
    assert!(!svc.is_empty());
}
