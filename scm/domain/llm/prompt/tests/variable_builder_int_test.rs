//! Tests for the `VariableBuilder` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptFactory, StdPromptFactory, VariableType};

/// @covers: VariableBuilder — builds a named, typed variable
#[test]
fn test_variable_builder_builds_named_typed() {
    let v = StdPromptFactory::variable_builder()
        .name("topic".to_string())
        .var_type(VariableType::Number)
        .build();
    assert_eq!(v.name, "topic");
    assert_eq!(v.var_type, VariableType::Number);
}

/// @covers: VariableBuilder — default value flips required off
#[test]
fn test_variable_builder_default_value_optional() {
    let v = StdPromptFactory::variable_builder()
        .default_value(serde_json::json!("x"))
        .build();
    assert!(!v.required);
    assert!(v.default.is_some());
}

/// @covers: VariableBuilder — description carried through
#[test]
fn test_variable_builder_description() {
    let v = StdPromptFactory::variable_builder()
        .description("doc".to_string())
        .build();
    assert_eq!(v.description.as_deref(), Some("doc"));
}
