#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ToolChoice` value type.

use edge_llm_agent::ToolChoice;

#[test]
fn test_tool_choice_variants_distinct() {
    assert_ne!(ToolChoice::Auto, ToolChoice::None);
    assert_ne!(ToolChoice::None, ToolChoice::Required);
}

#[test]
fn test_tool_choice_function_carries_name() {
    let choice = ToolChoice::Function {
        name: "search".to_string(),
    };
    match choice {
        ToolChoice::Function { name } => assert_eq!(name, "search"),
        _ => panic!("expected Function variant"),
    }
}

#[test]
fn test_tool_choice_serde_roundtrip() {
    let json = serde_json::to_string(&ToolChoice::Required).expect("serialize");
    let back: ToolChoice = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ToolChoice::Required);
}
