#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ToolCall` value type.

use edge_llm_agent::ToolCall;

fn sample() -> ToolCall {
    ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"q":"x"}"#.to_string(),
    }
}

#[test]
fn test_tool_call_fields_are_accessible() {
    let tc = sample();
    assert_eq!(tc.id, "1");
    assert_eq!(tc.name, "search");
}

#[test]
fn test_tool_call_equality() {
    assert_eq!(sample(), sample());
}

#[test]
fn test_tool_call_serde_roundtrip() {
    let json = serde_json::to_string(&sample()).expect("serialize");
    let back: ToolCall = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, sample());
}
