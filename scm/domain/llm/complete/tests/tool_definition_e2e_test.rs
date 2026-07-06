#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::ToolDefinition;
use serde_json::json;

#[test]
fn test_new_sets_all_fields() {
    let td = ToolDefinition::new("search", "Search the web", json!({"type": "object"}));
    assert_eq!(td.name, "search");
    assert_eq!(td.description, "Search the web");
}

#[test]
fn test_roundtrip_serialization() {
    let td = ToolDefinition::new("t", "d", json!({"a": 1}));
    let json = serde_json::to_string(&td).unwrap();
    let back: ToolDefinition = serde_json::from_str(&json).unwrap();
    assert_eq!(td, back);
}

#[test]
fn test_parameters_field_name_is_parameters_not_input_schema() {
    let td = ToolDefinition::new("t", "d", json!({}));
    let json = serde_json::to_value(&td).unwrap();
    assert!(
        json.get("parameters").is_some(),
        "field must be named 'parameters'"
    );
    assert!(json.get("input_schema").is_none());
}
