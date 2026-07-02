//! @covers PipelineConfig ConfigSection implementation and TOML deserialization.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use edge_domain_pipeline::PipelineConfig;
use swe_edge_configbuilder::ConfigSection;

// ── section_name ─────────────────────────────────────────────────────────────

/// @covers: section_name
#[test]
fn test_section_name_happy_returns_pipeline() {
    assert_eq!(PipelineConfig::section_name(), "pipeline");
}

/// @covers: section_name
#[test]
fn test_section_name_error_does_not_clash_with_other_domain_sections() {
    let name = PipelineConfig::section_name();
    assert_ne!(
        name, "validator",
        "pipeline section must not shadow validator"
    );
    assert_ne!(name, "policy", "pipeline section must not shadow policy");
    assert_ne!(name, "command", "pipeline section must not shadow command");
}

/// @covers: section_name
#[test]
fn test_section_name_edge_is_lowercase_toml_key() {
    let name = PipelineConfig::section_name();
    assert_eq!(name, name.to_lowercase());
}

// ── TOML deserialization ──────────────────────────────────────────────────────

#[test]
fn test_pipeline_config_toml_happy_all_fields() {
    let src = r#"
        timeout_per_step_ms   = 5000
        emit_lifecycle_events = true
        abort_on_error        = false
    "#;
    let config: PipelineConfig = toml::from_str(src).expect("valid TOML");
    assert_eq!(config.timeout_per_step, Some(Duration::from_millis(5000)));
    assert!(config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

#[test]
fn test_pipeline_config_toml_happy_empty_uses_defaults() {
    let config: PipelineConfig = toml::from_str("").expect("empty TOML uses defaults");
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(config.abort_on_error);
}

#[test]
fn test_pipeline_config_toml_happy_partial_overrides_only_specified() {
    let src = "abort_on_error = false";
    let config: PipelineConfig = toml::from_str(src).expect("partial TOML");
    assert!(config.timeout_per_step.is_none());
    assert!(!config.emit_lifecycle_events);
    assert!(!config.abort_on_error);
}

#[test]
fn test_pipeline_config_toml_edge_zero_timeout_ms() {
    let src = "timeout_per_step_ms = 0";
    let config: PipelineConfig = toml::from_str(src).expect("zero timeout is valid");
    assert_eq!(config.timeout_per_step, Some(Duration::ZERO));
}

#[test]
fn test_pipeline_config_toml_edge_large_timeout_ms() {
    let src = "timeout_per_step_ms = 3600000"; // 1 hour
    let config: PipelineConfig = toml::from_str(src).expect("large timeout is valid");
    assert_eq!(config.timeout_per_step, Some(Duration::from_secs(3600)));
}

#[test]
fn test_pipeline_config_toml_error_wrong_type_for_timeout() {
    let src = r#"timeout_per_step_ms = "not-a-number""#;
    let result: Result<PipelineConfig, _> = toml::from_str(src);
    assert!(
        result.is_err(),
        "string value for timeout_per_step_ms must be rejected"
    );
}
