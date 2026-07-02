//! Tests for the `VariableBuilder` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptBootstrap, StdPromptFactory, VariableKind};

/// @covers: VariableBuilder — builds a named, typed variable
#[test]
fn test_variable_builder_builds_named_typed() {
    let v = StdPromptFactory::variable_builder()
        .name("topic".to_string())
        .var_type(VariableKind::Number)
        .build();
    assert_eq!(v.name, "topic");
    assert_eq!(v.var_type, VariableKind::Number);
}

/// @covers: VariableBuilder — default value flips required off
#[test]
fn test_variable_builder_default_value_optional() {
    let v = StdPromptFactory::variable_builder()
        .default_value(serde_json::json!("x"))
        .build();
    assert!(!v.required);
    assert_eq!(v.default, Some(serde_json::json!("x").into()));
}

/// @covers: VariableBuilder — description carried through
#[test]
fn test_variable_builder_description() {
    let v = StdPromptFactory::variable_builder()
        .description("doc".to_string())
        .build();
    assert_eq!(v.description.as_deref(), Some("doc"));
}
