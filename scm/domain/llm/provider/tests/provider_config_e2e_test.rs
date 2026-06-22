//! Tests for `ProviderConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::ProviderConfig;
use std::io::Write as _;
use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection};
use tempfile::TempDir;

fn write_toml(dir: &TempDir, content: &str) {
    let path = dir.path().join("application.toml");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

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

/// @covers: ProviderConfig::load — deserializes all fields from TOML
#[test]
fn test_load_reads_all_fields_from_toml_happy() {
    let dir = TempDir::new().unwrap();
    write_toml(
        &dir,
        r#"
[llm.provider]
model = "claude-3-5-sonnet"
temperature = 0.3
api_base = "https://api.anthropic.com"
max_context_tokens = 200000
supports_vision = true
supports_functions = true
supports_streaming = true
"#,
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let cfg = ProviderConfig::load(&loader).unwrap();
    assert_eq!(cfg.model, "claude-3-5-sonnet");
    assert_eq!(cfg.temperature, 0.3);
    assert_eq!(cfg.api_base.as_deref(), Some("https://api.anthropic.com"));
    assert_eq!(cfg.max_context_tokens, 200_000);
    assert!(cfg.supports_vision);
    assert!(cfg.supports_functions);
    assert!(cfg.supports_streaming);
}

/// @covers: ProviderConfig::load — returns Default when section is absent
#[test]
fn test_load_returns_default_when_section_absent_edge() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, "[other]\nkey = 1");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let cfg = ProviderConfig::load(&loader).unwrap();
    assert!(cfg.model.is_empty());
    assert_eq!(cfg.max_context_tokens, 0);
}

/// @covers: ProviderConfig::load — returns parse error on type mismatch
#[test]
fn test_load_fails_on_type_mismatch_error() {
    let dir = TempDir::new().unwrap();
    write_toml(
        &dir,
        r#"
[llm.provider]
max_context_tokens = "not-a-number"
"#,
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result = ProviderConfig::load(&loader);
    assert!(result.is_err(), "expected parse error for wrong field type");
}
