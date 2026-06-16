//! Tests for AgentMetadataBuilder with fluent API.

use edge_domain_agent::{AgentMetadataBuilder, SkillMetadataBuilder};

#[test]
#[should_panic]
fn test_agent_metadata_builder_new_requires_fields() {
    // @covers AgentMetadataBuilder::new
    let builder = AgentMetadataBuilder::new();
    let _metadata = builder.build(); // Should panic
}

#[test]
fn test_agent_metadata_builder_default_exists() {
    // @covers AgentMetadataBuilder::default
    let _builder_default = AgentMetadataBuilder::default();
    assert!(true);
}

#[test]
fn test_agent_metadata_builder_id_sets_field() {
    // @covers AgentMetadataBuilder::id
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .build();
    assert_eq!(metadata.id, "agent-123");
}

#[test]
fn test_agent_metadata_builder_name_sets_field() {
    // @covers AgentMetadataBuilder::name
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .build();
    assert_eq!(metadata.name, "Test Agent");
}

#[test]
fn test_agent_metadata_builder_description_sets_field() {
    // @covers AgentMetadataBuilder::description
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .build();
    assert_eq!(metadata.description, "A test agent");
}

#[test]
fn test_agent_metadata_builder_version_sets_field() {
    // @covers AgentMetadataBuilder::version
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .build();
    assert_eq!(metadata.version, "1.0.0");
}

#[test]
fn test_agent_metadata_builder_skill_adds_single_skill() {
    // @covers AgentMetadataBuilder::skill
    let skill = SkillMetadataBuilder::new()
        .name("skill1")
        .description("First skill")
        .build();

    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .skill(skill)
        .build();

    assert_eq!(metadata.skills.len(), 1);
    assert_eq!(metadata.skills[0].name, "skill1");
}

#[test]
fn test_agent_metadata_builder_skill_adds_multiple_skills() {
    // @covers AgentMetadataBuilder::skill
    let skill1 = SkillMetadataBuilder::new()
        .name("skill1")
        .description("First skill")
        .build();
    let skill2 = SkillMetadataBuilder::new()
        .name("skill2")
        .description("Second skill")
        .build();

    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .skill(skill1)
        .skill(skill2)
        .build();

    assert_eq!(metadata.skills.len(), 2);
    assert_eq!(metadata.skills[0].name, "skill1");
    assert_eq!(metadata.skills[1].name, "skill2");
}

#[test]
fn test_agent_metadata_builder_skills_replaces_all_skills() {
    // @covers AgentMetadataBuilder::skills
    let skill1 = SkillMetadataBuilder::new()
        .name("skill1")
        .description("First skill")
        .build();
    let skill2 = SkillMetadataBuilder::new()
        .name("skill2")
        .description("Second skill")
        .build();
    let skills = vec![skill1, skill2];

    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .skills(skills)
        .build();

    assert_eq!(metadata.skills.len(), 2);
}

#[test]
fn test_agent_metadata_builder_pattern_adds_single_pattern() {
    // @covers AgentMetadataBuilder::pattern
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .pattern("react")
        .build();

    assert_eq!(metadata.patterns.len(), 1);
    assert_eq!(metadata.patterns[0], "react");
}

#[test]
fn test_agent_metadata_builder_pattern_adds_multiple_patterns() {
    // @covers AgentMetadataBuilder::pattern
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .pattern("react")
        .pattern("cot")
        .pattern("plan-execute")
        .build();

    assert_eq!(metadata.patterns.len(), 3);
    assert!(metadata.patterns.contains(&"react".to_string()));
    assert!(metadata.patterns.contains(&"cot".to_string()));
    assert!(metadata.patterns.contains(&"plan-execute".to_string()));
}

#[test]
fn test_agent_metadata_builder_patterns_replaces_all_patterns() {
    // @covers AgentMetadataBuilder::patterns
    let patterns = vec!["react".to_string(), "cot".to_string()];

    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test Agent")
        .description("A test agent")
        .version("1.0.0")
        .patterns(patterns)
        .build();

    assert_eq!(metadata.patterns.len(), 2);
}

#[test]
fn test_agent_metadata_builder_fluent_chain_complete() {
    // @covers AgentMetadataBuilder - fluent chain
    let skill = SkillMetadataBuilder::new()
        .name("search")
        .description("Web search skill")
        .async_execution(true)
        .build();

    let metadata = AgentMetadataBuilder::new()
        .id("agent-advanced")
        .name("Advanced Agent")
        .description("An agent with all capabilities")
        .version("2.0.0")
        .skill(skill)
        .pattern("react")
        .pattern("cot")
        .build();

    assert_eq!(metadata.id, "agent-advanced");
    assert_eq!(metadata.name, "Advanced Agent");
    assert_eq!(metadata.version, "2.0.0");
    assert_eq!(metadata.skills.len(), 1);
    assert_eq!(metadata.patterns.len(), 2);
}

#[test]
fn test_agent_metadata_builder_empty_skills_and_patterns_default() {
    // @covers AgentMetadataBuilder - empty collections default
    let metadata = AgentMetadataBuilder::new()
        .id("agent-minimal")
        .name("Minimal Agent")
        .description("Agent with no skills or patterns")
        .version("1.0.0")
        .build();

    assert_eq!(metadata.skills.len(), 0);
    assert_eq!(metadata.patterns.len(), 0);
}

#[test]
fn test_agent_metadata_builder_overwrites_required_fields() {
    // @covers AgentMetadataBuilder - field overwrite
    let metadata = AgentMetadataBuilder::new()
        .id("agent-1")
        .id("agent-2")
        .name("First Name")
        .name("Second Name")
        .description("First")
        .description("Second")
        .version("1.0")
        .version("2.0")
        .build();

    assert_eq!(metadata.id, "agent-2");
    assert_eq!(metadata.name, "Second Name");
    assert_eq!(metadata.version, "2.0");
}

#[test]
fn test_agent_metadata_builder_skills_replaces_previous() {
    // @covers AgentMetadataBuilder - skills replacement
    let skill1 = SkillMetadataBuilder::new()
        .name("skill1")
        .description("First")
        .build();
    let skill2 = SkillMetadataBuilder::new()
        .name("skill2")
        .description("Second")
        .build();

    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test")
        .description("Test agent")
        .version("1.0")
        .skill(skill1)
        .skills(vec![skill2])
        .build();

    assert_eq!(metadata.skills.len(), 1);
    assert_eq!(metadata.skills[0].name, "skill2");
}

#[test]
fn test_agent_metadata_builder_patterns_replaces_previous() {
    // @covers AgentMetadataBuilder - patterns replacement
    let metadata = AgentMetadataBuilder::new()
        .id("agent-123")
        .name("Test")
        .description("Test agent")
        .version("1.0")
        .pattern("react")
        .patterns(vec!["cot".to_string()])
        .build();

    assert_eq!(metadata.patterns.len(), 1);
    assert_eq!(metadata.patterns[0], "cot");
}

#[test]
fn test_agent_metadata_builder_accepts_string_refs() {
    // @covers AgentMetadataBuilder - Into<String> conversion
    let id_str = "agent-ref";
    let name_str = "Reference Agent";
    let desc_str = "Built from string references";
    let ver_str = "1.0.0";

    let metadata = AgentMetadataBuilder::new()
        .id(id_str)
        .name(name_str)
        .description(desc_str)
        .version(ver_str)
        .build();

    assert_eq!(metadata.id, id_str);
    assert_eq!(metadata.name, name_str);
    assert_eq!(metadata.description, desc_str);
    assert_eq!(metadata.version, ver_str);
}

#[test]
fn test_agent_metadata_builder_complex_scenario() {
    // @covers AgentMetadataBuilder - complex scenario
    let search_skill = SkillMetadataBuilder::new()
        .name("search")
        .description("Perform web searches")
        .async_execution(true)
        .build();

    let code_skill = SkillMetadataBuilder::new()
        .name("execute_code")
        .description("Execute code snippets")
        .async_execution(true)
        .long_running(true)
        .build();

    let metadata = AgentMetadataBuilder::new()
        .id("gpt-4-research")
        .name("Research Agent")
        .description("An agent designed for research tasks")
        .version("1.0.0")
        .skill(search_skill)
        .skill(code_skill)
        .pattern("react")
        .pattern("cot")
        .pattern("plan-execute")
        .build();

    assert_eq!(metadata.id, "gpt-4-research");
    assert_eq!(metadata.name, "Research Agent");
    assert_eq!(metadata.skills.len(), 2);
    assert_eq!(metadata.patterns.len(), 3);
    assert!(metadata.skills.iter().any(|s| s.name == "search"));
    assert!(metadata.skills.iter().any(|s| s.name == "execute_code"));
}
