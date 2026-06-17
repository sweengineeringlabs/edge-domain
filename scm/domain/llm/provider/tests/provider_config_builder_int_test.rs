//! Tests for `ProviderConfigBuilder`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ProviderConfigBuilder;

/// @covers: ProviderConfigBuilder::build — fluent overrides apply
#[test]
fn test_provider_config_builder_applies_overrides() {
    let config = ProviderConfigBuilder::new()
        .model("claude".to_string())
        .temperature(0.5)
        .max_context_tokens(200_000)
        .api_base("https://api.example.com".to_string())
        .supports_vision(true)
        .build();
    assert_eq!(config.model, "claude");
    assert_eq!(config.max_context_tokens, 200_000);
    assert_eq!(config.api_base.as_deref(), Some("https://api.example.com"));
    assert!(config.supports_vision);
}

/// @covers: ProviderConfigBuilder::default — empty model and no api base
#[test]
fn test_provider_config_builder_defaults() {
    let config = ProviderConfigBuilder::new().build();
    assert!(config.model.is_empty());
    assert!(config.api_base.is_none());
}
