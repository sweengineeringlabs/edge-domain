//! Scenario coverage for `ToolCallDelta`.

use edge_llm_complete::ToolCallDelta;

#[test]
fn test_tool_call_delta_new_sets_index_happy() {
    let delta = ToolCallDelta::new(0);
    assert_eq!(delta.index, 0);
}

#[test]
fn test_tool_call_delta_fields_default_to_none_error() {
    let delta = ToolCallDelta::new(1);
    assert!(delta.id.is_none() && delta.name.is_none() && delta.arguments.is_none());
}

/// @covers: with_id
#[test]
fn test_tool_call_delta_with_id_sets_id_edge() {
    let delta = ToolCallDelta::new(0).with_id("call-1");
    assert_eq!(delta.id, Some("call-1".to_string()));
}

/// @covers: with_name
#[test]
fn test_tool_call_delta_with_name_sets_name_happy() {
    let delta = ToolCallDelta::new(0).with_name("get_weather");
    assert_eq!(delta.name, Some("get_weather".to_string()));
}

/// @covers: with_arguments
#[test]
fn test_tool_call_delta_with_arguments_sets_arguments_happy() {
    let delta = ToolCallDelta::new(0).with_arguments(r#"{"city":"nyc"}"#);
    assert_eq!(delta.arguments, Some(r#"{"city":"nyc"}"#.to_string()));
}
