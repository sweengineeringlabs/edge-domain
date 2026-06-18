//! Tests for `ExecutionMode`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ExecutionMode;

/// @covers: ExecutionMode::is_streaming — streaming mode reports true
#[test]
fn test_is_streaming_true_for_streaming_happy() {
    assert!(ExecutionMode::Streaming.is_streaming());
}

/// @covers: ExecutionMode::is_streaming — async mode reports false
#[test]
fn test_is_streaming_false_for_async_error() {
    assert!(!ExecutionMode::Async.is_streaming());
}

/// @covers: ExecutionMode::is_async — async mode reports true
#[test]
fn test_is_async_true_for_async_happy() {
    assert!(ExecutionMode::Async.is_async());
}

/// @covers: ExecutionMode — serializes and deserializes correctly
#[test]
fn test_execution_mode_serde_roundtrip_edge() {
    let json = serde_json::to_string(&ExecutionMode::LongRunning).expect("serialize");
    let back: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ExecutionMode::LongRunning);
}
