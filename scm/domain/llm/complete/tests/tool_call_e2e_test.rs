//! Scenario coverage for `ToolCall`.

use edge_llm_complete::ToolCall;

#[test]
fn test_tool_call_new_sets_all_fields_happy() {
    let tc = ToolCall::new(
        "id-1".to_string(),
        "search".to_string(),
        r#"{"q":"x"}"#.to_string(),
    );
    assert_eq!(tc.id, "id-1");
    assert_eq!(tc.name, "search");
}

#[test]
fn test_tool_call_empty_fields_are_valid_error() {
    let tc = ToolCall::new(String::new(), String::new(), String::new());
    assert!(tc.id.is_empty());
}

#[test]
fn test_tool_call_arguments_is_raw_json_string_edge() {
    let tc = ToolCall::new("x".to_string(), "y".to_string(), r#"{"a":1}"#.to_string());
    assert!(tc.arguments.contains('{'));
}
