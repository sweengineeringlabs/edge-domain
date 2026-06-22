//! SAF facade tests — `Prompt` trait via `StaticPrompt`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    Prompt, PromptBootstrap, PromptMetadata, RenderContext, StdPromptFactory, Variable,
    VariableType,
};
use futures::executor::block_on;

fn prompt_with(template: &str, vars: Vec<Variable>) -> impl Prompt {
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vars,
    );
    StdPromptFactory::prompt(template.to_string(), metadata)
}

fn required(name: &str) -> Variable {
    Variable::new(name.to_string(), VariableType::String)
}

// --- render ---

/// @covers: Prompt::render — substitutes a provided variable
#[test]
fn test_render_substitutes_provided_variable_happy() {
    let p = prompt_with("Hi {{name}}", vec![required("name")]);
    let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    assert_eq!(block_on(p.render(&ctx)).expect("render"), "Hi Ada");
}

/// @covers: Prompt::render — errors when a required variable is missing
#[test]
fn test_render_missing_required_returns_error_error() {
    let p = prompt_with("Hi {{name}}", vec![required("name")]);
    let ctx = RenderContext::new();
    assert!(block_on(p.render(&ctx)).is_err());
}

/// @covers: Prompt::render — empty template with no vars renders empty
#[test]
fn test_render_empty_template_returns_empty_edge() {
    let p = prompt_with("", vec![]);
    let ctx = RenderContext::new();
    assert_eq!(block_on(p.render(&ctx)).expect("render"), "");
}

// --- metadata ---

/// @covers: Prompt::metadata — reports the configured id
#[test]
fn test_metadata_reports_id_happy() {
    assert_eq!(prompt_with("x", vec![]).metadata().id, "greet");
}

/// @covers: Prompt::metadata — variable list carried through
#[test]
fn test_metadata_carries_variables_error() {
    let p = prompt_with("x", vec![required("a")]);
    assert_eq!(p.metadata().variables.len(), 1);
}

/// @covers: Prompt::metadata — empty variable set is reported as empty
#[test]
fn test_metadata_empty_variables_edge() {
    assert!(prompt_with("x", vec![]).metadata().variables.is_empty());
}

// --- validate ---

/// @covers: Prompt::validate — balanced braces validate cleanly
#[test]
fn test_validate_balanced_braces_ok_happy() {
    assert!(prompt_with("Hi {{name}}", vec![required("name")])
        .validate()
        .is_ok());
}

/// @covers: Prompt::validate — unbalanced braces are rejected
#[test]
fn test_validate_unbalanced_braces_rejected_error() {
    assert!(prompt_with("Hi {{name}", vec![]).validate().is_err());
}

/// @covers: Prompt::validate — template with no placeholders is valid
#[test]
fn test_validate_no_placeholders_ok_edge() {
    assert!(prompt_with("plain text", vec![]).validate().is_ok());
}

// --- variable_type ---

/// @covers: Prompt::variable_type — reports the declared type
#[test]
fn test_variable_type_reports_declared_happy() {
    let p = prompt_with("x", vec![required("name")]);
    assert_eq!(p.variable_type("name"), Some(VariableType::String));
}

/// @covers: Prompt::variable_type — unknown variable returns None
#[test]
fn test_variable_type_unknown_returns_none_error() {
    let p = prompt_with("x", vec![required("name")]);
    assert_eq!(p.variable_type("missing"), None);
}

/// @covers: Prompt::variable_type — empty name with no vars returns None
#[test]
fn test_variable_type_empty_name_none_edge() {
    assert_eq!(prompt_with("x", vec![]).variable_type(""), None);
}

// --- cache ---

/// @covers: Prompt::cache — builds an entry keyed by template id
#[test]
fn test_cache_keys_by_template_id_happy() {
    let p = prompt_with("x", vec![]);
    let entry = p.cache(&RenderContext::new(), "rendered".to_string());
    assert!(entry.key.starts_with("greet::"));
}

/// @covers: Prompt::cache — token count reflects rendered length
#[test]
fn test_cache_token_count_matches_rendered_error() {
    let p = prompt_with("x", vec![]);
    let entry = p.cache(&RenderContext::new(), "abcd".to_string());
    assert_eq!(entry.token_count, 4);
}

/// @covers: Prompt::cache — empty rendered yields zero token count
#[test]
fn test_cache_empty_rendered_zero_tokens_edge() {
    let p = prompt_with("x", vec![]);
    let entry = p.cache(&RenderContext::new(), String::new());
    assert_eq!(entry.token_count, 0);
}
