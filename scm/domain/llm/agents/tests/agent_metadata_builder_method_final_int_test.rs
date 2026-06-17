#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for AgentMetadata::builder() method.

use edge_llm_agent::AgentMetadata;

#[test]
fn test_agent_metadata_builder_method_returns_builder() {
    // @covers AgentMetadata::builder
    let builder = AgentMetadata::builder();
    let metadata = builder
        .id("test-agent")
        .name("Test")
        .description("Test agent")
        .version("1.0")
        .build();
    assert_eq!(metadata.id, "test-agent");
}

#[test]
fn test_agent_metadata_builder_method_fluent_chain() {
    // @covers AgentMetadata::builder - fluent pattern
    let metadata = AgentMetadata::builder()
        .id("agent-1")
        .name("Agent One")
        .description("First agent")
        .version("1.0.0")
        .build();

    assert_eq!(metadata.name, "Agent One");
    assert_eq!(metadata.version, "1.0.0");
}

#[test]
fn test_agent_metadata_builder_method_empty_collections() {
    // @covers AgentMetadata::builder - default empty collections
    let metadata = AgentMetadata::builder()
        .id("minimal")
        .name("Minimal")
        .description("Minimal metadata")
        .version("0.1")
        .build();

    assert_eq!(metadata.skills.len(), 0);
    assert_eq!(metadata.patterns.len(), 0);
}
