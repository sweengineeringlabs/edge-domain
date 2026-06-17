#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ParameterDocumentationBuilder`.

use edge_llm_agent::ParameterDocumentationBuilder;
use serde_json::json;

#[test]
fn test_parameter_documentation_builder_builds_required_fields() {
    let doc = ParameterDocumentationBuilder::new("q", "query", "string", true).build();
    assert_eq!(doc.name, "q");
    assert_eq!(doc.param_type, "string");
    assert!(doc.required);
}

#[test]
fn test_parameter_documentation_builder_sets_default_value() {
    let doc = ParameterDocumentationBuilder::new("q", "d", "string", false)
        .default_value(json!("fallback"))
        .build();
    assert_eq!(doc.default, Some(json!("fallback")));
}

#[test]
fn test_parameter_documentation_builder_accumulates_examples() {
    let doc = ParameterDocumentationBuilder::new("q", "d", "string", true)
        .example(json!("a"))
        .examples(vec![json!("b"), json!("c")])
        .validation_rules("non-empty")
        .build();
    assert_eq!(doc.examples.len(), 2);
    assert_eq!(doc.validation_rules.as_deref(), Some("non-empty"));
}
