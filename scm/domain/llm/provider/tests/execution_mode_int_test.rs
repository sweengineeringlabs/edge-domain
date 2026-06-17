//! Tests for the `ExecutionMode` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ExecutionMode;

/// @covers: ExecutionMode::is_streaming
#[test]
fn test_is_streaming_true_for_streaming() {
    assert!(ExecutionMode::Streaming.is_streaming());
}

/// @covers: ExecutionMode::is_streaming
#[test]
fn test_is_streaming_false_for_async() {
    assert!(!ExecutionMode::Async.is_streaming());
}

/// @covers: ExecutionMode::is_async
#[test]
fn test_is_async_true_for_async() {
    assert!(ExecutionMode::Async.is_async());
}

/// @covers: ExecutionMode — serde round-trip
#[test]
fn test_execution_mode_serde_roundtrip() {
    let json = serde_json::to_string(&ExecutionMode::LongRunning).expect("serialize");
    let back: ExecutionMode = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, ExecutionMode::LongRunning);
}

/// @covers: ExecutionMode — equality and hashing
#[test]
fn test_execution_mode_equality() {
    assert_eq!(ExecutionMode::Async, ExecutionMode::Async);
    assert_ne!(ExecutionMode::Async, ExecutionMode::Streaming);
}
