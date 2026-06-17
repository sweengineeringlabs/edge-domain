//! Tests for the `StreamDelta` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{StreamDelta, ToolCallDelta};

/// @covers: StreamDelta::text — carries text content
#[test]
fn test_text_carries_content() {
    let delta = StreamDelta::text("hello".to_string());
    assert_eq!(delta.content.as_deref(), Some("hello"));
    assert!(!delta.is_empty());
}

/// @covers: StreamDelta::empty — has no content or tool calls
#[test]
fn test_empty_is_empty() {
    assert!(StreamDelta::empty().is_empty());
}

/// @covers: StreamDelta::tool_calls — carries tool-call deltas
#[test]
fn test_tool_calls_carry_deltas() {
    let delta = StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]);
    assert_eq!(delta.tool_calls.len(), 1);
    assert!(!delta.is_empty());
}

/// @covers: StreamDelta — serde round-trip
#[test]
fn test_stream_delta_serde_roundtrip() {
    let delta = StreamDelta::text("hi".to_string());
    let json = serde_json::to_string(&delta).expect("serialize");
    let back: StreamDelta = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.content.as_deref(), Some("hi"));
}
