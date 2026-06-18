//! Tests for `ExecutionConfigBuilder` setter methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionConfigBuilder, ExecutionMode};

/// @covers: ExecutionConfigBuilder::max_tokens_per_call — sets the token cap
#[test]
fn test_max_tokens_per_call() {
    let c = ExecutionConfigBuilder::new().max_tokens_per_call(1024).build();
    assert_eq!(c.max_tokens_per_call, 1024);
}

/// @covers: ExecutionConfigBuilder::max_tokens_per_call — zero is the minimum
#[test]
fn test_max_tokens_per_call_zero_minimum_edge() {
    let c = ExecutionConfigBuilder::new().max_tokens_per_call(0).build();
    assert_eq!(c.max_tokens_per_call, 0);
}

/// @covers: ExecutionConfigBuilder::timeout_per_step — sets the timeout
#[test]
fn test_timeout_per_step() {
    let c = ExecutionConfigBuilder::new().timeout_per_step(5000).build();
    assert_eq!(c.timeout_per_step, 5000);
}

/// @covers: ExecutionConfigBuilder::cache_enabled — enables caching
#[test]
fn test_cache_enabled() {
    let c = ExecutionConfigBuilder::new().cache_enabled(true).build();
    assert!(c.cache_enabled);
}

/// @covers: ExecutionConfigBuilder::cache_enabled — disabled by default
#[test]
fn test_cache_enabled_default_false_edge() {
    let c = ExecutionConfigBuilder::new().build();
    assert!(!c.cache_enabled);
}

/// @covers: ExecutionConfigBuilder::streaming_enabled — enables streaming
#[test]
fn test_streaming_enabled() {
    let c = ExecutionConfigBuilder::new().streaming_enabled(true).build();
    assert!(c.streaming_enabled);
}

/// @covers: ExecutionConfigBuilder::execution_mode — sets the mode
#[test]
fn test_execution_mode() {
    let c = ExecutionConfigBuilder::new().execution_mode(ExecutionMode::Streaming).build();
    assert_eq!(c.execution_mode, ExecutionMode::Streaming);
}

/// @covers: ExecutionConfigBuilder::build — produces a valid ExecutionConfig
#[test]
fn test_build() {
    let c = ExecutionConfigBuilder::new().build();
    assert_eq!(c.execution_mode, ExecutionMode::Async);
}

/// @covers: ExecutionConfigBuilder::build — zero max tokens is preserved
#[test]
fn test_build_zero_tokens_edge() {
    let c = ExecutionConfigBuilder::new().max_tokens_per_call(0).build();
    assert_eq!(c.max_tokens_per_call, 0);
}
