//! Tests for `ProviderConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ProviderConfig;

/// @covers: ProviderConfig::new — sets core fields correctly
#[test]
fn test_new_sets_core_fields_happy() {
    let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
    assert_eq!(config.model, "gpt-4");
    assert_eq!(config.temperature, 0.7);
    assert_eq!(config.max_context_tokens, 8192);
}

/// @covers: ProviderConfig::new — defaults capabilities to off
#[test]
fn test_new_defaults_capabilities_off_error() {
    let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
    assert!(!config.supports_vision);
    assert!(!config.supports_functions);
}

/// @covers: ProviderConfig — serializes and deserializes correctly
#[test]
fn test_provider_config_serde_roundtrip_edge() {
    let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
    let json = serde_json::to_string(&config).expect("serialize");
    let back: ProviderConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.model, "gpt-4");
}
