//! Tests for `StreamDelta`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{StreamDelta, ToolCallDelta};

/// @covers: StreamDelta::text — carries content and is non-empty
#[test]
fn test_text_carries_content_happy() {
    let delta = StreamDelta::text("hello".to_string());
    assert_eq!(delta.content.as_deref(), Some("hello"));
    assert!(!delta.is_empty());
}

/// @covers: StreamDelta::empty — is_empty returns true
#[test]
fn test_empty_is_empty_error() {
    assert!(StreamDelta::empty().is_empty());
}

/// @covers: StreamDelta::tool_calls — carries tool call deltas
#[test]
fn test_tool_calls_carry_deltas_happy() {
    let delta = StreamDelta::tool_calls(vec![ToolCallDelta::new(0)]);
    assert_eq!(delta.tool_calls.len(), 1);
    assert!(!delta.is_empty());
}

/// @covers: StreamDelta::is_empty — text delta is non-empty
#[test]
fn test_is_empty_text_delta_false_edge() {
    assert!(!StreamDelta::text("x".to_string()).is_empty());
}

/// @covers: StreamDelta — serializes and deserializes correctly
#[test]
fn test_stream_delta_serde_roundtrip_edge() {
    let delta = StreamDelta::text("hi".to_string());
    let json = serde_json::to_string(&delta).expect("serialize");
    let back: StreamDelta = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.content.as_deref(), Some("hi"));
}
