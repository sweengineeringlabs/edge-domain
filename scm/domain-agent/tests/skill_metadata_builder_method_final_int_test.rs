//! Tests for SkillMetadata::builder() method.

use edge_domain_agent::SkillMetadata;

#[test]
fn test_skill_metadata_builder_method_returns_builder() {
    // @covers SkillMetadata::builder
    let builder = SkillMetadata::builder();
    let metadata = builder
        .name("test_skill")
        .description("Test skill")
        .build();
    assert_eq!(metadata.name, "test_skill");
}

#[test]
fn test_skill_metadata_builder_method_fluent_chain() {
    // @covers SkillMetadata::builder - fluent pattern
    let metadata = SkillMetadata::builder()
        .name("search")
        .description("Web search capability")
        .async_execution(true)
        .long_running(false)
        .build();

    assert_eq!(metadata.name, "search");
    assert!(metadata.async_execution);
    assert!(!metadata.long_running);
}

#[test]
fn test_skill_metadata_builder_method_with_schemas() {
    // @covers SkillMetadata::builder - schema support
    let metadata = SkillMetadata::builder()
        .name("complex_skill")
        .description("Skill with schemas")
        .input_schema(r#"{"type": "object"}"#)
        .output_schema(r#"{"type": "string"}"#)
        .build();

    assert!(metadata.input_schema.is_some());
    assert!(metadata.output_schema.is_some());
}
