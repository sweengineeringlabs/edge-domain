//! Tests for `ExecutionConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionConfig, ExecutionMode};

/// @covers: ExecutionConfig::timeout — converts timeout to duration
#[test]
fn test_timeout_converts_to_duration_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
    assert_eq!(config.timeout().as_millis(), 30_000);
}

/// @covers: ExecutionConfig::supports_streaming — true when streaming enabled
#[test]
fn test_supports_streaming_true_when_enabled_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    assert!(config.supports_streaming());
}

/// @covers: ExecutionConfig::supports_streaming — false when streaming disabled
#[test]
fn test_supports_streaming_false_when_disabled_error() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Streaming);
    assert!(!config.supports_streaming());
}

/// @covers: ExecutionConfig — serializes and deserializes correctly
#[test]
fn test_execution_config_serde_roundtrip_edge() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
    let json = serde_json::to_string(&config).expect("serialize");
    let back: ExecutionConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.max_tokens_per_call, 4096);
}
