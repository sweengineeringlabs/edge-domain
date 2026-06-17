//! Tests for the `StreamChunk` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{FinishReason, StreamChunk, StreamDelta};

/// @covers: StreamChunk::is_terminal — true when a finish reason is set
#[test]
fn test_is_terminal_true_with_finish_reason() {
    let chunk = StreamChunk::new(
        "c1".to_string(),
        StreamDelta::empty(),
        Some(FinishReason::Stop),
    );
    assert!(chunk.is_terminal());
}

/// @covers: StreamChunk::is_terminal — false without a finish reason
#[test]
fn test_is_terminal_false_without_finish_reason() {
    let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
    assert!(!chunk.is_terminal());
}

/// @covers: StreamChunk — serde round-trip
#[test]
fn test_stream_chunk_serde_roundtrip() {
    let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
    let json = serde_json::to_string(&chunk).expect("serialize");
    let back: StreamChunk = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.id, "c1");
}
