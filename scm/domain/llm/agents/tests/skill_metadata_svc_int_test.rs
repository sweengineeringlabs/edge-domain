#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for SkillMetadata type re-export via skill_metadata_svc.rs.

use edge_llm_agent::SkillMetadata;

/// @covers: SkillMetadata type re-export
#[test]
fn test_svc_skill_metadata_happy_type_can_be_constructed() {
    let metadata = SkillMetadata {
        name: "analyze".to_string(),
        description: "Analyze input".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.name, "analyze");
}

/// @covers: SkillMetadata type re-export — all fields
#[test]
fn test_svc_skill_metadata_happy_all_fields_accessible() {
    let metadata = SkillMetadata {
        name: "code_review".to_string(),
        description: "Review code".to_string(),
        input_schema: Some("{}".to_string()),
        output_schema: Some("{}".to_string()),
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.name, "code_review");
    assert_eq!(metadata.description, "Review code");
    assert!(metadata.input_schema.is_some());
    assert!(metadata.output_schema.is_some());
    assert!(metadata.async_execution);
    assert!(!metadata.long_running);
}

/// @covers: SkillMetadata type re-export — name field
#[test]
fn test_svc_skill_metadata_happy_name_field_stores_value() {
    let metadata = SkillMetadata {
        name: "custom_skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.name, "custom_skill");
}

/// @covers: SkillMetadata type re-export — description field
#[test]
fn test_svc_skill_metadata_happy_description_field_stores_value() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Custom description".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.description, "Custom description");
}

/// @covers: SkillMetadata type re-export — input_schema Some
#[test]
fn test_svc_skill_metadata_happy_input_schema_with_value() {
    let schema = r#"{"type": "object"}"#.to_string();
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: Some(schema.clone()),
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.input_schema.is_some());
    assert_eq!(metadata.input_schema.unwrap(), schema);
}

/// @covers: SkillMetadata type re-export — input_schema None
#[test]
fn test_svc_skill_metadata_happy_input_schema_none() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.input_schema.is_none());
}

/// @covers: SkillMetadata type re-export — output_schema Some
#[test]
fn test_svc_skill_metadata_happy_output_schema_with_value() {
    let schema = r#"{"type": "string"}"#.to_string();
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: Some(schema.clone()),
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.output_schema.is_some());
    assert_eq!(metadata.output_schema.unwrap(), schema);
}

/// @covers: SkillMetadata type re-export — output_schema None
#[test]
fn test_svc_skill_metadata_happy_output_schema_none() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.output_schema.is_none());
}

/// @covers: SkillMetadata type re-export — async_execution true
#[test]
fn test_svc_skill_metadata_happy_async_execution_true() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.async_execution);
}

/// @covers: SkillMetadata type re-export — async_execution false
#[test]
fn test_svc_skill_metadata_happy_async_execution_false() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: false,
        long_running: false,
    };
    assert!(!metadata.async_execution);
}

/// @covers: SkillMetadata type re-export — long_running true
#[test]
fn test_svc_skill_metadata_happy_long_running_true() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: true,
    };
    assert!(metadata.long_running);
}

/// @covers: SkillMetadata type re-export — long_running false
#[test]
fn test_svc_skill_metadata_happy_long_running_false() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    assert!(!metadata.long_running);
}

/// @covers: SkillMetadata type re-export — Clone trait
#[test]
fn test_svc_skill_metadata_happy_can_be_cloned() {
    let original = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: Some("{}".to_string()),
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    let cloned = original.clone();
    assert_eq!(cloned.name, "skill");
    assert_eq!(cloned.description, "Desc");
}

/// @covers: SkillMetadata type re-export — Debug trait
#[test]
fn test_svc_skill_metadata_happy_debug_format_available() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    let debug_str = format!("{:?}", metadata);
    assert!(debug_str.contains("skill"));
}

/// @covers: SkillMetadata type re-export — both schemas present
#[test]
fn test_svc_skill_metadata_happy_both_schemas_present() {
    let metadata = SkillMetadata {
        name: "skill".to_string(),
        description: "Desc".to_string(),
        input_schema: Some(r#"{"type": "object"}"#.to_string()),
        output_schema: Some(r#"{"type": "string"}"#.to_string()),
        async_execution: true,
        long_running: false,
    };
    assert!(metadata.input_schema.is_some());
    assert!(metadata.output_schema.is_some());
}

/// @covers: SkillMetadata type re-export — edge case empty strings
#[test]
fn test_svc_skill_metadata_edge_empty_string_fields() {
    let metadata = SkillMetadata {
        name: String::new(),
        description: String::new(),
        input_schema: Some(String::new()),
        output_schema: Some(String::new()),
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.name, "");
    assert_eq!(metadata.description, "");
    assert!(metadata.input_schema.is_some());
}

/// @covers: SkillMetadata type re-export — all flags together
#[test]
fn test_svc_skill_metadata_happy_all_flag_combinations() {
    let combinations = vec![(true, true), (true, false), (false, true), (false, false)];

    for (async_exec, long_run) in combinations {
        let metadata = SkillMetadata {
            name: "skill".to_string(),
            description: "Desc".to_string(),
            input_schema: None,
            output_schema: None,
            async_execution: async_exec,
            long_running: long_run,
        };
        assert_eq!(metadata.async_execution, async_exec);
        assert_eq!(metadata.long_running, long_run);
    }
}
