#![allow(clippy::unwrap_used, clippy::expect_used)]
//! SAF tests for the `SkillMetadataBuilder` service export.

use edge_llm_agent::{SkillMetadataBuilder, SKILL_METADATA_BUILDER_SVC};

#[test]
fn test_skill_metadata_builder_svc_constant_value() {
    assert_eq!(SKILL_METADATA_BUILDER_SVC, "skill_metadata_builder");
}

#[test]
fn test_skill_metadata_builder_svc_builds_metadata() {
    let metadata = SkillMetadataBuilder::new()
        .name("svc_skill")
        .description("built via svc export")
        .build();
    assert_eq!(metadata.name, "svc_skill");
}

#[test]
fn test_skill_metadata_builder_svc_defaults_empty() {
    let metadata = SkillMetadataBuilder::default().build();
    assert!(metadata.name.is_empty());
}
