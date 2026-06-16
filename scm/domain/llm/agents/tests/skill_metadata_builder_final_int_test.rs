//! Tests for SkillMetadataBuilder with fluent API.

use edge_llm_agent::SkillMetadataBuilder;

#[test]
#[should_panic]
fn test_skill_metadata_builder_requires_name() {
    // @covers SkillMetadataBuilder::new and SkillMetadataBuilder::build
    let builder = SkillMetadataBuilder::new();
    let _metadata = builder.build(); // Should panic because name is required
}

#[test]
fn test_skill_metadata_builder_default_exists() {
    // @covers SkillMetadataBuilder::default
    let _builder_default = SkillMetadataBuilder::default();
    assert!(true);
}

#[test]
fn test_skill_metadata_builder_name_sets_field() {
    // @covers SkillMetadataBuilder::name
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .build();
    assert_eq!(metadata.name, "test_skill");
}

#[test]
fn test_skill_metadata_builder_description_sets_field() {
    // @covers SkillMetadataBuilder::description
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .build();
    assert_eq!(metadata.description, "Test description");
}

#[test]
fn test_skill_metadata_builder_input_schema_sets_field() {
    // @covers SkillMetadataBuilder::input_schema
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .input_schema(r#"{"type": "object"}"#)
        .build();
    assert_eq!(metadata.input_schema, Some(r#"{"type": "object"}"#.to_string()));
}

#[test]
fn test_skill_metadata_builder_output_schema_sets_field() {
    // @covers SkillMetadataBuilder::output_schema
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .output_schema(r#"{"type": "string"}"#)
        .build();
    assert_eq!(metadata.output_schema, Some(r#"{"type": "string"}"#.to_string()));
}

#[test]
fn test_skill_metadata_builder_async_execution_true() {
    // @covers SkillMetadataBuilder::async_execution
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .async_execution(true)
        .build();
    assert!(metadata.async_execution);
}

#[test]
fn test_skill_metadata_builder_async_execution_false() {
    // @covers SkillMetadataBuilder::async_execution
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .async_execution(false)
        .build();
    assert!(!metadata.async_execution);
}

#[test]
fn test_skill_metadata_builder_long_running_true() {
    // @covers SkillMetadataBuilder::long_running
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .long_running(true)
        .build();
    assert!(metadata.long_running);
}

#[test]
fn test_skill_metadata_builder_long_running_false() {
    // @covers SkillMetadataBuilder::long_running
    let metadata = SkillMetadataBuilder::new()
        .name("test_skill")
        .description("Test description")
        .long_running(false)
        .build();
    assert!(!metadata.long_running);
}

#[test]
fn test_skill_metadata_builder_fluent_chain_all_fields() {
    // @covers SkillMetadataBuilder - fluent chain
    let metadata = SkillMetadataBuilder::new()
        .name("complex_skill")
        .description("A complex skill with all fields set")
        .input_schema(r#"{"type": "object", "properties": {"param": {"type": "string"}}}"#)
        .output_schema(r#"{"type": "object", "properties": {"result": {"type": "string"}}}"#)
        .async_execution(true)
        .long_running(true)
        .build();

    assert_eq!(metadata.name, "complex_skill");
    assert_eq!(metadata.description, "A complex skill with all fields set");
    assert!(metadata.input_schema.is_some());
    assert!(metadata.output_schema.is_some());
    assert!(metadata.async_execution);
    assert!(metadata.long_running);
}

#[test]
fn test_skill_metadata_builder_defaults_optional_fields_to_none() {
    // @covers SkillMetadataBuilder - optional field defaults
    let metadata = SkillMetadataBuilder::new()
        .name("minimal_skill")
        .description("Minimal skill without schemas")
        .build();

    assert_eq!(metadata.input_schema, None);
    assert_eq!(metadata.output_schema, None);
}

#[test]
fn test_skill_metadata_builder_defaults_boolean_fields_to_false() {
    // @covers SkillMetadataBuilder - boolean field defaults
    let metadata = SkillMetadataBuilder::new()
        .name("sync_skill")
        .description("Synchronous, short-running skill")
        .build();

    assert!(!metadata.async_execution);
    assert!(!metadata.long_running);
}

#[test]
fn test_skill_metadata_builder_empty_schemas_valid() {
    // @covers SkillMetadataBuilder - edge case empty schemas
    let metadata = SkillMetadataBuilder::new()
        .name("edge_skill")
        .description("Skill with empty schemas")
        .input_schema("")
        .output_schema("")
        .build();

    assert_eq!(metadata.input_schema, Some(String::new()));
    assert_eq!(metadata.output_schema, Some(String::new()));
}

#[test]
fn test_skill_metadata_builder_overwrites_previous_values() {
    // @covers SkillMetadataBuilder - field overwrite
    let metadata = SkillMetadataBuilder::new()
        .name("first_name")
        .name("second_name")
        .description("First description")
        .description("Second description")
        .build();

    assert_eq!(metadata.name, "second_name");
    assert_eq!(metadata.description, "Second description");
}

#[test]
fn test_skill_metadata_builder_accepts_string_refs() {
    // @covers SkillMetadataBuilder - Into<String> conversion
    let name_str = "ref_skill";
    let desc_str = "A skill built from string references";
    let metadata = SkillMetadataBuilder::new()
        .name(name_str)
        .description(desc_str)
        .build();

    assert_eq!(metadata.name, name_str);
    assert_eq!(metadata.description, desc_str);
}
