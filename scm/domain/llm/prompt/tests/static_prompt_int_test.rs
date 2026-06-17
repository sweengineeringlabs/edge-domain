//! Tests for the `StaticPrompt` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    Prompt, PromptMetadata, RenderContext, StaticPrompt, Variable, VariableType,
};
use futures::executor::block_on;

fn build(template: &str) -> StaticPrompt {
    let var = Variable::new("name".to_string(), VariableType::String);
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vec![var],
    );
    StaticPrompt::new(template.to_string(), metadata)
}

/// @covers: StaticPrompt::new — renders substituted variables
#[test]
fn test_static_prompt_renders_substitution() {
    let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    assert_eq!(
        block_on(build("Hi {{name}}").render(&ctx)).expect("render"),
        "Hi Ada"
    );
}

/// @covers: StaticPrompt — clone preserves metadata identity
#[test]
fn test_static_prompt_clone_preserves_id() {
    assert_eq!(build("x").clone().metadata().id, "greet");
}

/// @covers: StaticPrompt — reports declared variable type
#[test]
fn test_static_prompt_variable_type() {
    assert_eq!(build("x").variable_type("name"), Some(VariableType::String));
}
