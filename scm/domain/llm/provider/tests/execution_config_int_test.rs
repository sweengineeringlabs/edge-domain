//! Tests for the `ExecutionConfig` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionConfig, ExecutionMode};

/// @covers: ExecutionConfig::timeout — converts millis to a Duration
#[test]
fn test_timeout_converts_to_duration() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
    assert_eq!(config.timeout().as_millis(), 30_000);
}

/// @covers: ExecutionConfig::supports_streaming — true when enabled and streaming
#[test]
fn test_supports_streaming_true_when_enabled() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    assert!(config.supports_streaming());
}

/// @covers: ExecutionConfig::supports_streaming — false when disabled
#[test]
fn test_supports_streaming_false_when_disabled() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Streaming);
    assert!(!config.supports_streaming());
}

/// @covers: ExecutionConfig — serde round-trip
#[test]
fn test_execution_config_serde_roundtrip() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
    let json = serde_json::to_string(&config).expect("serialize");
    let back: ExecutionConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.max_tokens_per_call, 4096);
}
