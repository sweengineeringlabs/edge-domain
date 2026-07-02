//! End-to-end contract tests for the `Prompt` trait, exercised through the
//! crate's reference implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CacheBuildRequest, Prompt, PromptMetadata, PromptMetadataRequest, PromptVariableKindRequest,
    RenderContext, RenderRequest, StaticPrompt, TemplateValidationRequest, Variable, VariableKind,
};
use futures::executor::block_on;

fn prompt() -> StaticPrompt {
    let var = Variable::new("name".to_string(), VariableKind::String);
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vec![var],
    );
    StaticPrompt::new("Hi {{name}}".to_string(), metadata)
}

/// @covers: Prompt::render
#[test]
fn test_render_substitutes_variable_happy() {
    let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(prompt().render(RenderRequest { context: &ctx }))
        .expect("render ok")
        .rendered;
    assert_eq!(out, "Hi Ada");
}

/// @covers: Prompt::render
#[test]
fn test_render_missing_required_variable_returns_error() {
    let ctx = RenderContext::new();
    assert!(block_on(prompt().render(RenderRequest { context: &ctx })).is_err());
}

/// @covers: Prompt::metadata
#[test]
fn test_metadata_reports_configured_id() {
    let result = prompt()
        .metadata(PromptMetadataRequest)
        .expect("metadata ok");
    assert_eq!(result.id, "greet");
}

/// @covers: Prompt::validate
#[test]
fn test_validate_balanced_braces_ok() {
    assert_eq!(prompt().validate(TemplateValidationRequest), Ok(()));
}

/// @covers: Prompt::variable_kind
#[test]
fn test_variable_kind_reports_declared_type() {
    let result = prompt()
        .variable_kind(PromptVariableKindRequest { name: "name" })
        .expect("variable_kind ok");
    assert_eq!(result.kind, Some(VariableKind::String));
}

/// @covers: Prompt::cache
#[test]
fn test_cache_builds_entry_with_rendered_text() {
    let ctx = RenderContext::new();
    let result = prompt()
        .cache(CacheBuildRequest {
            context: &ctx,
            rendered: "Hi Ada".to_string(),
        })
        .expect("cache ok");
    assert_eq!(result.rendered, "Hi Ada");
}
