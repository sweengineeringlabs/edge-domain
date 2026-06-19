//! Tests for the `StaticProvider` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_llm_complete::{Completer, NoopCompleter};
use edge_llm_provider::{ModelFamily, ModelInfo, Provider, ProviderConfig, StaticProvider};
use futures::executor::block_on;

fn build(model: &str) -> StaticProvider {
    let config = ProviderConfig::new(model.to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        model.to_string(),
        model.to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    StaticProvider::new(config, info, Arc::new(NoopCompleter))
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

// --- completer ---

/// @covers: StaticProvider::completer — returns the delegate completer
#[test]
fn test_static_provider_completer_returns_delegate_happy() {
    let p = build("claude");
    let c = p.completer();
    assert!(block_on(c.list_models()).is_ok());
}

/// @covers: StaticProvider::completer — noop completer has no supported models
#[test]
fn test_static_provider_completer_noop_no_models_error() {
    let c = build("claude").completer();
    assert!(c.supported_models().is_empty());
}

/// @covers: StaticProvider::completer — clone shares the same Arc
#[test]
fn test_static_provider_completer_clone_shares_arc_edge() {
    let p = build("claude");
    let a: Arc<dyn Completer> = p.completer();
    let b: Arc<dyn Completer> = p.clone().completer();
    assert!(Arc::ptr_eq(&a, &b));
}
