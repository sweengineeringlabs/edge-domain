//! Integration tests for AgentMetadata type re-export via agent_metadata_svc.rs.

use edge_domain_agent::{AgentMetadata, SkillMetadata};

/// @covers: AgentMetadata type re-export
#[test]
fn test_svc_agent_metadata_happy_type_can_be_constructed() {
    let metadata = AgentMetadata {
        id: "agent1".to_string(),
        name: "Agent One".to_string(),
        description: "First agent".to_string(),
        version: "1.0.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.id, "agent1");
}

/// @covers: AgentMetadata type re-export — all fields
#[test]
fn test_svc_agent_metadata_happy_all_fields_accessible() {
    let metadata = AgentMetadata {
        id: "test_agent".to_string(),
        name: "Test Agent".to_string(),
        description: "A test agent".to_string(),
        version: "0.1.0".to_string(),
        skills: vec![],
        patterns: vec!["react".to_string()],
    };
    assert_eq!(metadata.id, "test_agent");
    assert_eq!(metadata.name, "Test Agent");
    assert_eq!(metadata.description, "A test agent");
    assert_eq!(metadata.version, "0.1.0");
    assert_eq!(metadata.patterns.len(), 1);
}

/// @covers: AgentMetadata type re-export — id field
#[test]
fn test_svc_agent_metadata_happy_id_field_stores_value() {
    let metadata = AgentMetadata {
        id: "unique_id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.id, "unique_id");
}

/// @covers: AgentMetadata type re-export — name field
#[test]
fn test_svc_agent_metadata_happy_name_field_stores_value() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Custom Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.name, "Custom Name");
}

/// @covers: AgentMetadata type re-export — description field
#[test]
fn test_svc_agent_metadata_happy_description_field_stores_value() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Custom description".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.description, "Custom description");
}

/// @covers: AgentMetadata type re-export — version field
#[test]
fn test_svc_agent_metadata_happy_version_field_stores_value() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "2.5.3".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.version, "2.5.3");
}

/// @covers: AgentMetadata type re-export — skills field empty
#[test]
fn test_svc_agent_metadata_happy_skills_field_empty() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.skills.len(), 0);
}

/// @covers: AgentMetadata type re-export — skills field with values
#[test]
fn test_svc_agent_metadata_happy_skills_field_with_values() {
    let skill = SkillMetadata {
        name: "analyze".to_string(),
        description: "Analyze data".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![skill],
        patterns: vec![],
    };
    assert_eq!(metadata.skills.len(), 1);
    assert_eq!(metadata.skills[0].name, "analyze");
}

/// @covers: AgentMetadata type re-export — patterns field
#[test]
fn test_svc_agent_metadata_happy_patterns_field() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec!["react".to_string(), "cot".to_string()],
    };
    assert_eq!(metadata.patterns.len(), 2);
    assert!(metadata.patterns.contains(&"react".to_string()));
    assert!(metadata.patterns.contains(&"cot".to_string()));
}

/// @covers: AgentMetadata type re-export — Clone trait
#[test]
fn test_svc_agent_metadata_happy_can_be_cloned() {
    let original = AgentMetadata {
        id: "agent1".to_string(),
        name: "Agent".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    let cloned = original.clone();
    assert_eq!(cloned.id, "agent1");
    assert_eq!(cloned.name, "Agent");
}

/// @covers: AgentMetadata type re-export — Debug trait
#[test]
fn test_svc_agent_metadata_happy_debug_format_available() {
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns: vec![],
    };
    let debug_str = format!("{:?}", metadata);
    assert!(debug_str.contains("id"));
}

/// @covers: AgentMetadata type re-export — multiple skills
#[test]
fn test_svc_agent_metadata_happy_multiple_skills() {
    let skill1 = SkillMetadata {
        name: "skill1".to_string(),
        description: "First".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    let skill2 = SkillMetadata {
        name: "skill2".to_string(),
        description: "Second".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: false,
        long_running: true,
    };
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![skill1, skill2],
        patterns: vec![],
    };
    assert_eq!(metadata.skills.len(), 2);
}

/// @covers: AgentMetadata type re-export — edge case empty strings
#[test]
fn test_svc_agent_metadata_edge_empty_string_fields() {
    let metadata = AgentMetadata {
        id: String::new(),
        name: String::new(),
        description: String::new(),
        version: String::new(),
        skills: vec![],
        patterns: vec![],
    };
    assert_eq!(metadata.id, "");
    assert_eq!(metadata.name, "");
    assert_eq!(metadata.version, "");
}

/// @covers: AgentMetadata type re-export — many patterns
#[test]
fn test_svc_agent_metadata_happy_many_patterns() {
    let patterns = vec![
        "react".to_string(),
        "cot".to_string(),
        "plan-execute".to_string(),
        "multi-turn".to_string(),
    ];
    let metadata = AgentMetadata {
        id: "id".to_string(),
        name: "Name".to_string(),
        description: "Desc".to_string(),
        version: "1.0".to_string(),
        skills: vec![],
        patterns,
    };
    assert_eq!(metadata.patterns.len(), 4);
}
