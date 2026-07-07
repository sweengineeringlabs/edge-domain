//! SAF facade tests — `Prompt` trait via `StaticPrompt`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CacheBuildRequest, Prompt, PromptMetadata, PromptMetadataRequest, PromptVariableKindRequest,
    RenderContext, RenderRequest, StaticPrompt, TemplateValidationRequest, Variable, VariableKind,
};
use futures::executor::block_on;

fn prompt_with(template: &str, vars: Vec<Variable>) -> impl Prompt {
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vars,
    );
    StaticPrompt::new(template.to_string(), metadata)
}

fn required(name: &str) -> Variable {
    Variable::new(name.to_string(), VariableKind::String)
}

// --- render ---

/// @covers: Prompt::render — substitutes a provided variable
#[test]
fn test_render_substitutes_provided_variable_happy() {
    let p = prompt_with("Hi {{name}}", vec![required("name")]);
    let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    assert_eq!(
        block_on(p.render(RenderRequest { context: &ctx }))
            .expect("render")
            .rendered,
        "Hi Ada"
    );
}

/// @covers: Prompt::render — errors when a required variable is missing
#[test]
fn test_render_missing_required_returns_error_error() {
    let p = prompt_with("Hi {{name}}", vec![required("name")]);
    let ctx = RenderContext::new();
    assert!(block_on(p.render(RenderRequest { context: &ctx })).is_err());
}

/// @covers: Prompt::render — empty template with no vars renders empty
#[test]
fn test_render_empty_template_returns_empty_edge() {
    let p = prompt_with("", vec![]);
    let ctx = RenderContext::new();
    assert_eq!(
        block_on(p.render(RenderRequest { context: &ctx }))
            .expect("render")
            .rendered,
        ""
    );
}

// --- metadata ---

/// @covers: Prompt::metadata — reports the configured id
#[test]
fn test_metadata_reports_id_happy() {
    assert_eq!(
        prompt_with("x", vec![])
            .metadata(PromptMetadataRequest)
            .expect("metadata")
            .id,
        "greet"
    );
}

/// @covers: Prompt::metadata — variable list carried through
#[test]
fn test_metadata_carries_variables_error() {
    let p = prompt_with("x", vec![required("a")]);
    assert_eq!(
        p.metadata(PromptMetadataRequest)
            .expect("metadata")
            .variables
            .len(),
        1
    );
}

/// @covers: Prompt::metadata — empty variable set is reported as empty
#[test]
fn test_metadata_empty_variables_edge() {
    assert!(prompt_with("x", vec![])
        .metadata(PromptMetadataRequest)
        .expect("metadata")
        .variables
        .is_empty());
}

// --- validate ---

/// @covers: Prompt::validate — balanced braces validate cleanly
#[test]
fn test_validate_balanced_braces_ok_happy() {
    assert_eq!(
        prompt_with("Hi {{name}}", vec![required("name")]).validate(TemplateValidationRequest),
        Ok(())
    );
}

/// @covers: Prompt::validate — unbalanced braces are rejected
#[test]
fn test_validate_unbalanced_braces_rejected_error() {
    assert!(prompt_with("Hi {{name}", vec![])
        .validate(TemplateValidationRequest)
        .is_err());
}

/// @covers: Prompt::validate — template with no placeholders is valid
#[test]
fn test_validate_no_placeholders_ok_edge() {
    assert_eq!(
        prompt_with("plain text", vec![]).validate(TemplateValidationRequest),
        Ok(())
    );
}

// --- variable_kind ---

/// @covers: Prompt::variable_kind — reports the declared type
#[test]
fn test_variable_kind_reports_declared_happy() {
    let p = prompt_with("x", vec![required("name")]);
    assert_eq!(
        p.variable_kind(PromptVariableKindRequest { name: "name" })
            .expect("variable_kind")
            .kind,
        Some(VariableKind::String)
    );
}

/// @covers: Prompt::variable_kind — unknown variable returns None
#[test]
fn test_variable_kind_unknown_returns_none_error() {
    let p = prompt_with("x", vec![required("name")]);
    assert_eq!(
        p.variable_kind(PromptVariableKindRequest { name: "missing" })
            .expect("variable_kind")
            .kind,
        None
    );
}

/// @covers: Prompt::variable_kind — empty name with no vars returns None
#[test]
fn test_variable_kind_empty_name_none_edge() {
    assert_eq!(
        prompt_with("x", vec![])
            .variable_kind(PromptVariableKindRequest { name: "" })
            .expect("variable_kind")
            .kind,
        None
    );
}

// --- cache ---

/// @covers: Prompt::cache — builds an entry keyed by template id
#[test]
fn test_cache_keys_by_template_id_happy() {
    let p = prompt_with("x", vec![]);
    let entry = p
        .cache(CacheBuildRequest {
            context: &RenderContext::new(),
            rendered: "rendered".to_string(),
        })
        .expect("cache");
    assert!(entry.key.starts_with("greet::"));
}

/// @covers: Prompt::cache — token count reflects rendered length
#[test]
fn test_cache_token_count_matches_rendered_error() {
    let p = prompt_with("x", vec![]);
    let entry = p
        .cache(CacheBuildRequest {
            context: &RenderContext::new(),
            rendered: "abcd".to_string(),
        })
        .expect("cache");
    assert_eq!(entry.token_count, 4);
}

/// @covers: Prompt::cache — empty rendered yields zero token count
#[test]
fn test_cache_empty_rendered_zero_tokens_edge() {
    let p = prompt_with("x", vec![]);
    let entry = p
        .cache(CacheBuildRequest {
            context: &RenderContext::new(),
            rendered: String::new(),
        })
        .expect("cache");
    assert_eq!(entry.token_count, 0);
}
