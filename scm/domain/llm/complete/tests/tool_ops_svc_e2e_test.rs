//! Scenario coverage for the `tool_ops_svc` SAF surface.

use edge_llm_complete::{NoopCompleter, ToolCall, ToolOps, TOOL_OPS_SVC};

#[test]
fn test_tool_ops_svc_constant_is_expected_value_happy() {
    assert_eq!(TOOL_OPS_SVC, "tool_ops");
}

#[test]
fn test_tool_ops_svc_constant_is_nonempty_error() {
    assert!(!TOOL_OPS_SVC.is_empty());
}

#[test]
fn test_tool_ops_available_tools_is_empty_for_noop_edge() {
    assert!(NoopCompleter.available_tools().is_empty());
}
