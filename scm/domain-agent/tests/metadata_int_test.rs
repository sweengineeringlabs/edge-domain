//! Integration tests for AgentMetadata and SkillMetadata types.

use edge_domain_agent::{AgentMetadata, SkillMetadata};

#[test]
fn type_agent_metadata_happy_construction() {
    let metadata = AgentMetadata {
        id: "test_agent".to_string(),
        name: "Test Agent".to_string(),
        description: "A test agent".to_string(),
        version: "0.1.0".to_string(),
        skills: vec![],
        patterns: vec!["react".to_string()],
    };
    assert_eq!(metadata.id, "test_agent");
}

#[test]
fn type_agent_metadata_happy_with_skills() {
    let skill_meta = SkillMetadata {
        name: "analyze".to_string(),
        description: "Analyze input".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: true,
        long_running: false,
    };
    let metadata = AgentMetadata {
        id: "test".to_string(),
        name: "Test".to_string(),
        description: "Test".to_string(),
        version: "1.0.0".to_string(),
        skills: vec![skill_meta],
        patterns: vec![],
    };
    assert_eq!(metadata.skills.len(), 1);
}

#[test]
fn type_skill_metadata_happy_construction() {
    let metadata = SkillMetadata {
        name: "code_review".to_string(),
        description: "Review code".to_string(),
        input_schema: Some("{}".to_string()),
        output_schema: Some("{}".to_string()),
        async_execution: true,
        long_running: false,
    };
    assert_eq!(metadata.name, "code_review");
    assert!(metadata.async_execution);
}

#[test]
fn type_skill_metadata_edge_optional_schemas() {
    let metadata = SkillMetadata {
        name: "test".to_string(),
        description: "Test".to_string(),
        input_schema: None,
        output_schema: None,
        async_execution: false,
        long_running: true,
    };
    assert!(metadata.input_schema.is_none());
    assert!(metadata.long_running);
}
