//! Tests for the `ToolCallDelta` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ToolCallDelta;

/// @covers: ToolCallDelta::new — sets the index and leaves fields unset
#[test]
fn test_new_sets_index() {
    let delta = ToolCallDelta::new(3);
    assert_eq!(delta.index, 3);
    assert!(delta.id.is_none());
    assert!(delta.name.is_none());
}

/// @covers: ToolCallDelta — clone preserves index
#[test]
fn test_tool_call_delta_clone() {
    let delta = ToolCallDelta::new(1);
    assert_eq!(delta.clone().index, 1);
}

/// @covers: ToolCallDelta — serde round-trip
#[test]
fn test_tool_call_delta_serde_roundtrip() {
    let delta = ToolCallDelta::new(2);
    let json = serde_json::to_string(&delta).expect("serialize");
    let back: ToolCallDelta = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.index, 2);
}
