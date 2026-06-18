//! Tests for `ProviderConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ProviderConfig;
use swe_edge_configbuilder::ConfigSection;

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

/// @covers: ProviderConfig::section_name — TOML section key is stable
#[test]
fn test_provider_config_section_name_stable_happy() {
    assert_eq!(ProviderConfig::section_name(), "llm.provider");
}

/// @covers: ProviderConfig — Default produces empty config
#[test]
fn test_provider_config_default_is_empty_edge() {
    let cfg = ProviderConfig::default();
    assert!(cfg.model.is_empty());
    assert!(!cfg.supports_vision);
}
