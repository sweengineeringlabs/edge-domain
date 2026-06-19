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

#[test]
fn test_tool_call_delta_with_id_sets_id_edge() {
    let delta = ToolCallDelta::new(0).with_id("call-1");
    assert_eq!(delta.id, Some("call-1".to_string()));
}
