//! Tests for `ToolCallDeltaBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ToolCallDeltaBuilder;

/// @covers: ToolCallDeltaBuilder::build — fluent overrides apply
#[test]
fn test_tool_call_delta_builder_applies_overrides() {
    let delta = ToolCallDeltaBuilder::new(2)
        .id("call_1".to_string())
        .name("search".to_string())
        .arguments("{\"q\":\"x\"}".to_string())
        .build();
    assert_eq!(delta.index, 2);
    assert_eq!(delta.id.as_deref(), Some("call_1"));
    assert_eq!(delta.name.as_deref(), Some("search"));
    assert!(delta.arguments.is_some());
}

/// @covers: ToolCallDeltaBuilder::new — index set, fields unset by default
#[test]
fn test_tool_call_delta_builder_defaults() {
    let delta = ToolCallDeltaBuilder::new(0).build();
    assert_eq!(delta.index, 0);
    assert!(delta.id.is_none());
}
