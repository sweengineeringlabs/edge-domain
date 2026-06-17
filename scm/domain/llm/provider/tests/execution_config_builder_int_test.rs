//! Tests for `ExecutionConfigBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionConfigBuilder, ExecutionMode};

/// @covers: ExecutionConfigBuilder::build — fluent overrides apply
#[test]
fn test_execution_config_builder_applies_overrides() {
    let config = ExecutionConfigBuilder::new()
        .max_tokens_per_call(2048)
        .timeout_per_step(10_000)
        .execution_mode(ExecutionMode::Streaming)
        .streaming_enabled(true)
        .build();
    assert_eq!(config.max_tokens_per_call, 2048);
    assert_eq!(config.timeout_per_step, 10_000);
    assert!(config.supports_streaming());
}

/// @covers: ExecutionConfigBuilder::default — sane defaults
#[test]
fn test_execution_config_builder_defaults() {
    let config = ExecutionConfigBuilder::new().build();
    assert_eq!(config.max_tokens_per_call, 4096);
    assert_eq!(config.execution_mode, ExecutionMode::Async);
}
