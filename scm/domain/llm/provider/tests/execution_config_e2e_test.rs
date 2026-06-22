//! Tests for `ExecutionConfig`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{ExecutionConfig, ExecutionMode};
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

/// @covers: ExecutionConfig::timeout — converts timeout_per_step millis to Duration
#[test]
fn test_timeout_converts_to_duration_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Async);
    assert_eq!(config.timeout().as_millis(), 30_000);
}

/// @covers: ExecutionConfig::supports_streaming — true when mode is Streaming and enabled
#[test]
fn test_supports_streaming_true_when_enabled_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    assert!(config.supports_streaming());
}

/// @covers: ExecutionConfig::supports_streaming — false when streaming_enabled is false
#[test]
fn test_supports_streaming_false_when_disabled_error() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Streaming);
    assert!(!config.supports_streaming());
}

/// @covers: ExecutionConfig::section_name — TOML key is stable
#[test]
fn test_section_name_is_llm_execution_happy() {
    assert_eq!(ExecutionConfig::section_name(), "llm.execution");
}

/// @covers: ExecutionConfig::load — deserializes all fields from TOML
#[test]
fn test_load_reads_all_fields_from_toml_happy() {
    let dir = TempDir::new().unwrap();
    write_toml(
        &dir,
        r#"
[llm.execution]
max_tokens_per_call = 2048
timeout_per_step = 15000
cache_enabled = true
streaming_enabled = true
execution_mode = "Streaming"
"#,
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let cfg = ExecutionConfig::load(&loader).unwrap();
    assert_eq!(cfg.max_tokens_per_call, 2048);
    assert_eq!(cfg.timeout_per_step, 15_000);
    assert!(cfg.cache_enabled);
    assert!(cfg.streaming_enabled);
    assert_eq!(cfg.execution_mode, ExecutionMode::Streaming);
}

/// @covers: ExecutionConfig::load — returns Default when section is absent
#[test]
fn test_load_returns_default_when_section_absent_edge() {
    let dir = TempDir::new().unwrap();
    write_toml(&dir, "[other]\nkey = 1");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let cfg = ExecutionConfig::load(&loader).unwrap();
    assert_eq!(cfg.max_tokens_per_call, 0);
    assert!(!cfg.cache_enabled);
}

/// @covers: ExecutionConfig::load — returns parse error on type mismatch
#[test]
fn test_load_fails_on_type_mismatch_error() {
    let dir = TempDir::new().unwrap();
    write_toml(
        &dir,
        r#"
[llm.execution]
max_tokens_per_call = "not-a-number"
"#,
    );
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    let result = ExecutionConfig::load(&loader);
    assert!(result.is_err(), "expected parse error for wrong field type");
}
