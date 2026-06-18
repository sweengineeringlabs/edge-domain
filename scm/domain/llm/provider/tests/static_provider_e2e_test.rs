//! Tests for the `StaticProvider` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfo, Provider, ProviderConfig, StaticProvider};

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
