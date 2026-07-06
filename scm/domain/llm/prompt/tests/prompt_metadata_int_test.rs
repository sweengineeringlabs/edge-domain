//! Tests for the `PromptMetadata` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptMetadata, Variable, VariableKind};

fn meta(vars: Vec<Variable>) -> PromptMetadata {
    PromptMetadata::new("id".to_string(), "name".to_string(), "1".to_string(), vars)
}

/// @covers: PromptMetadata::new — sets core fields
#[test]
fn test_prompt_metadata_new_sets_fields() {
    let m = meta(vec![]);
    assert_eq!(m.id, "id");
    assert_eq!(m.version, "1");
}

/// @covers: PromptMetadata::required_variables — filters required only
#[test]
fn test_prompt_metadata_required_variables() {
    let req = Variable::new("a".to_string(), VariableKind::String);
    let opt = Variable::with_default("b".to_string(), VariableKind::String, serde_json::json!(1));
    let m = meta(vec![req, opt]);
    assert_eq!(m.required_variables().len(), 1);
}

/// @covers: PromptMetadata::optional_variables — filters optional only
#[test]
fn test_prompt_metadata_optional_variables() {
    let opt = Variable::with_default("b".to_string(), VariableKind::String, serde_json::json!(1));
    let m = meta(vec![opt]);
    assert_eq!(m.optional_variables().len(), 1);
}

/// @covers: PromptMetadata::with_tag — appends a tag
#[test]
fn test_prompt_metadata_with_tag() {
    let m = meta(vec![]).with_tag("system".to_string());
    assert_eq!(m.tags, vec!["system".to_string()]);
}
