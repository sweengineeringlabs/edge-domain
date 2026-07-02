//! Tests for `ToolDefinition`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{JsonValue, ToolDefinition};
use serde_json::json;

/// @covers: ToolDefinition::new — stores all fields
#[test]
fn test_tool_definition_new_happy() {
    let schema = json!({ "type": "object", "properties": { "query": { "type": "string" } } });
    let tool = ToolDefinition::new("search", "Search the web", schema.clone());
    assert_eq!(tool.name, "search");
    assert_eq!(tool.description, "Search the web");
    assert_eq!(tool.input_schema, JsonValue::from(schema));
}

/// @covers: ToolDefinition — serializes and deserializes correctly
#[test]
fn test_tool_definition_serde_roundtrip_edge() {
    let tool = ToolDefinition::new("calc", "Calculate", json!({ "type": "object" }));
    let json = serde_json::to_string(&tool).expect("serialize");
    let back: ToolDefinition = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.name, "calc");
    assert_eq!(back.description, "Calculate");
}

/// @covers: ToolDefinition — empty schema is valid
#[test]
fn test_tool_definition_empty_schema_edge() {
    let tool = ToolDefinition::new("noop", "Does nothing", json!({}));
    assert!(matches!(tool.input_schema, JsonValue::Object(_)));
}
