//! Tests for `StreamChunk`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{FinishReason, StreamChunk, StreamDelta};

/// @covers: StreamChunk::is_terminal — true when finish reason present
#[test]
fn test_is_terminal_true_with_finish_reason_happy() {
    let chunk = StreamChunk::new(
        "c1".to_string(),
        StreamDelta::empty(),
        Some(FinishReason::Stop),
    );
    assert!(chunk.is_terminal());
}

/// @covers: StreamChunk::is_terminal — false when no finish reason
#[test]
fn test_is_terminal_false_without_finish_reason_error() {
    let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
    assert!(!chunk.is_terminal());
}

/// @covers: StreamChunk — serializes and deserializes correctly
#[test]
fn test_stream_chunk_serde_roundtrip_edge() {
    let chunk = StreamChunk::new("c1".to_string(), StreamDelta::text("hi".to_string()), None);
    let json = serde_json::to_string(&chunk).expect("serialize");
    let back: StreamChunk = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.id, "c1");
}
