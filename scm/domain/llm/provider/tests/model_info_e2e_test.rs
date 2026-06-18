//! Tests for `ModelInfo`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ModelFamily, ModelInfo};
use swe_edge_configbuilder::{ConfigLoaderFactory, ConfigSection};
use std::io::Write as _;
use tempfile::TempDir;

fn write_toml(dir: &TempDir, content: &str) {
    let path = dir.path().join("application.toml");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

/// @covers: ModelInfo::new — sets core fields correctly
#[test]
fn test_new_sets_core_fields_happy() {
    let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
    assert_eq!(info.id, "gpt-4");
    assert_eq!(info.context_window, 128_000);
    assert_eq!(info.family, ModelFamily::OpenAI);
}

/// @covers: ModelInfo::new — defaults capabilities to off
#[test]
fn test_new_defaults_capabilities_off_error() {
    let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
    assert!(!info.supports_vision);
    assert!(!info.supports_functions);
}

/// @covers: ModelInfo::section_name — TOML section key is stable
#[test]
fn test_section_name_is_llm_model_happy() {
    assert_eq!(ModelInfo::section_name(), "llm.model");
}

/// @covers: ModelInfo::load — deserializes all fields from TOML
#[test]
fn test_load_reads_all_fields_from_toml_happy() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, r#"
[llm.model]
id = "claude-3-5-sonnet-20241022"
name = "Claude 3.5 Sonnet"
family = "anthropic"
context_window = 200000
supports_vision = true
supports_functions = true
supports_streaming = true
training_cutoff = "2024-04"
"#);
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let info = ModelInfo::load(&loader).unwrap();
    assert_eq!(info.id, "claude-3-5-sonnet-20241022");
    assert_eq!(info.family, ModelFamily::Anthropic);
    assert_eq!(info.context_window, 200_000);
    assert!(info.supports_vision);
    assert_eq!(info.training_cutoff.as_deref(), Some("2024-04"));
}

/// @covers: ModelInfo::load — returns Default when section is absent
#[test]
fn test_load_returns_default_when_section_absent_edge() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, "[other]\nkey = 1");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let info = ModelInfo::load(&loader).unwrap();
    assert!(info.id.is_empty());
    assert_eq!(info.context_window, 0);
}

/// @covers: ModelInfo::load — returns parse error on type mismatch
#[test]
fn test_load_fails_on_type_mismatch_error() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, r#"
[llm.model]
context_window = "not-a-number"
"#);
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result = ModelInfo::load(&loader);
    assert!(result.is_err(), "expected parse error for wrong field type");
}
