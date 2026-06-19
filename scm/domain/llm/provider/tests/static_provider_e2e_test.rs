//! Tests for the `StaticProvider` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    CompletionInput, ExecutionConfig, ExecutionMode, ModelFamily, ModelInfo, Provider,
    ProviderConfig, StaticProvider,
};
use futures::executor::block_on;
use futures::StreamExt;

fn build(model: &str) -> StaticProvider {
    let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        model.to_string(),
        model.to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    StaticProvider::new(config, info)
}

/// @covers: StaticProvider::new — reports the configured name
#[test]
fn test_static_provider_reports_name() {
    assert_eq!(build("claude").name(), "claude");
}

/// @covers: StaticProvider — healthy when configured with a model
#[test]
fn test_static_provider_healthy_with_model() {
    assert!(build("claude").health_check().is_ok());
}

/// @covers: StaticProvider — clone preserves identity
#[test]
fn test_static_provider_clone_preserves_name() {
    let provider = build("claude");
    assert_eq!(provider.clone().name(), "claude");
}

fn input() -> CompletionInput {
    CompletionInput::simple(
        "ping",
        ExecutionConfig::new(4096, 30_000, false, false, ExecutionMode::Async),
    )
}

// --- complete (noop) ---

/// @covers: StaticProvider::complete — noop returns empty reasoning
#[test]
fn test_static_provider_complete_noop_returns_empty_reasoning_happy() {
    let result = block_on(build("claude").complete(&input())).expect("noop should not error");
    assert!(result.reasoning.is_empty());
}

/// @covers: StaticProvider::complete — noop has no action
#[test]
fn test_static_provider_complete_noop_has_no_action_error() {
    let result = block_on(build("claude").complete(&input())).expect("noop should not error");
    assert!(result.action.is_none());
}

/// @covers: StaticProvider::complete — noop returns zero confidence
#[test]
fn test_static_provider_complete_noop_zero_confidence_edge() {
    let result = block_on(build("claude").complete(&input())).expect("noop should not error");
    assert_eq!(result.confidence, 0.0);
}

// --- stream (noop) ---

/// @covers: StaticProvider::stream — noop returns empty stream
#[test]
fn test_static_provider_stream_noop_returns_empty_stream_happy() {
    let stream = block_on(build("claude").stream(&input())).expect("noop should not error");
    let chunks: Vec<_> = block_on(stream.collect::<Vec<_>>());
    assert!(chunks.is_empty());
}

/// @covers: StaticProvider::stream — noop stream yields no error chunks
#[test]
fn test_static_provider_stream_noop_no_errors_error() {
    let stream = block_on(build("claude").stream(&input())).expect("noop should not error");
    assert!(block_on(stream.collect::<Vec<_>>()).is_empty());
}

/// @covers: StaticProvider::stream — noop is callable multiple times without panic
#[test]
fn test_static_provider_stream_noop_repeatable_edge() {
    let p = build("claude");
    let _ = block_on(p.stream(&input())).expect("first call ok");
    let _ = block_on(p.stream(&input())).expect("second call ok");
}
