//! Tests for the `PromptMetadataBuilder` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptFactory, StdPromptFactory, Variable, VariableType};

/// @covers: PromptMetadataBuilder — builds with id, name, version
#[test]
fn test_prompt_metadata_builder_core_fields() {
    let m = StdPromptFactory::prompt_metadata_builder()
        .id("t".to_string())
        .name("T".to_string())
        .version("2".to_string())
        .build();
    assert_eq!(m.id, "t");
    assert_eq!(m.version, "2");
}

/// @covers: PromptMetadataBuilder — variables carried through
#[test]
fn test_prompt_metadata_builder_variables() {
    let var = Variable::new("a".to_string(), VariableType::String);
    let m = StdPromptFactory::prompt_metadata_builder()
        .variables(vec![var])
        .build();
    assert_eq!(m.variables.len(), 1);
}

/// @covers: PromptMetadataBuilder — base token count carried through
#[test]
fn test_prompt_metadata_builder_base_token_count() {
    let m = StdPromptFactory::prompt_metadata_builder()
        .base_token_count(42)
        .build();
    assert_eq!(m.base_token_count, 42);
}
