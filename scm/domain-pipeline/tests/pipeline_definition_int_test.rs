//! @covers PipelineDefinition — ConfigSection impl and TOML deserialization.

use std::time::Duration;

use edge_domain_pipeline::PipelineDefinition;
use swe_edge_configbuilder::ConfigSection;

// ── section_name ──────────────────────────────────────────────────────────────

/// @covers: section_name
#[test]
fn test_section_name_happy_returns_pipeline() {
    assert_eq!(PipelineDefinition::section_name(), "pipeline");
}

/// @covers: section_name
#[test]
fn test_section_name_error_does_not_clash_with_other_domain_sections() {
    let name = PipelineDefinition::section_name();
    assert_ne!(name, "validator");
    assert_ne!(name, "policy");
    assert_ne!(name, "step_registry");
}

/// @covers: section_name
#[test]
fn test_section_name_edge_is_lowercase() {
    let name = PipelineDefinition::section_name();
    assert_eq!(name, name.to_lowercase());
}

// ── TOML deserialization ──────────────────────────────────────────────────────

#[test]
fn test_pipeline_definition_toml_happy_all_fields() {
    let src = r#"
        abort_on_error        = true
        timeout_per_step_ms   = 3000
        emit_lifecycle_events = true
        steps = ["step-a", "step-b", "step-c"]
    "#;
    let def: PipelineDefinition = toml::from_str(src).expect("valid TOML");
    assert!(def.config.abort_on_error);
    assert_eq!(def.config.timeout_per_step, Some(Duration::from_millis(3000)));
    assert!(def.config.emit_lifecycle_events);
    assert_eq!(def.steps, vec!["step-a", "step-b", "step-c"]);
}

#[test]
fn test_pipeline_definition_toml_happy_empty_uses_defaults() {
    let def: PipelineDefinition = toml::from_str("").expect("empty TOML uses defaults");
    assert!(def.steps.is_empty());
    assert!(def.config.abort_on_error);
    assert!(def.config.timeout_per_step.is_none());
}

#[test]
fn test_pipeline_definition_toml_happy_steps_only() {
    let src = r#"steps = ["validate", "enrich"]"#;
    let def: PipelineDefinition = toml::from_str(src).expect("valid TOML");
    assert_eq!(def.steps, vec!["validate", "enrich"]);
    assert!(def.config.abort_on_error); // default
}

#[test]
fn test_pipeline_definition_toml_edge_empty_steps_list() {
    let src = "steps = []";
    let def: PipelineDefinition = toml::from_str(src).expect("empty steps list is valid");
    assert!(def.steps.is_empty());
}

#[test]
fn test_pipeline_definition_toml_error_wrong_type_for_steps() {
    let src = "steps = 42";
    let result: Result<PipelineDefinition, _> = toml::from_str(src);
    assert!(result.is_err(), "integer steps must be rejected");
}
