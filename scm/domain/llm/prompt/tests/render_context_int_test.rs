//! Tests for the `RenderContext` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::RenderContext;

/// @covers: RenderContext::with_variable — stores and retrieves a value
#[test]
fn test_render_context_with_variable() {
    let ctx = RenderContext::new().with_variable("a".to_string(), serde_json::json!(1));
    assert_eq!(ctx.get_variable("a"), Some(&serde_json::json!(1)));
}

/// @covers: RenderContext::with_metadata — stores and retrieves metadata
#[test]
fn test_render_context_with_metadata() {
    let ctx = RenderContext::new().with_metadata("user".to_string(), "ada".to_string());
    assert_eq!(ctx.get_metadata("user"), Some("ada"));
}

/// @covers: RenderContext::has_all_variables — true only when all present
#[test]
fn test_render_context_has_all_variables() {
    let ctx = RenderContext::new().with_variable("a".to_string(), serde_json::json!(1));
    assert!(ctx.has_all_variables(&["a"]));
    assert!(!ctx.has_all_variables(&["a", "b"]));
}

/// @covers: RenderContext::variable_count — counts stored variables
#[test]
fn test_render_context_variable_count() {
    let ctx = RenderContext::new()
        .with_variable("a".to_string(), serde_json::json!(1))
        .with_variable("b".to_string(), serde_json::json!(2));
    assert_eq!(ctx.variable_count(), 2);
}
