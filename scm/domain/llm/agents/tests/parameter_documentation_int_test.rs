#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ParameterDocumentation` value type.

use edge_llm_agent::ParameterDocumentation;

#[test]
fn test_parameter_documentation_new_sets_required_fields() {
    let doc = ParameterDocumentation::new(
        "query".to_string(),
        "the query".to_string(),
        "string".to_string(),
        true,
    );
    assert_eq!(doc.name, "query");
    assert!(doc.required);
}

#[test]
fn test_parameter_documentation_new_defaults_optional_fields() {
    let doc = ParameterDocumentation::new(
        "q".to_string(),
        "d".to_string(),
        "string".to_string(),
        false,
    );
    assert!(doc.default.is_none());
    assert!(doc.examples.is_empty());
    assert!(doc.validation_rules.is_none());
}

#[test]
fn test_parameter_documentation_clone_preserves_type() {
    let doc =
        ParameterDocumentation::new("n".to_string(), "d".to_string(), "number".to_string(), true);
    assert_eq!(doc.clone().param_type, "number");
}
