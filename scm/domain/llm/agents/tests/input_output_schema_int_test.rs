#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `InputOutputSchema` value type.

use edge_llm_agent::InputOutputSchema;
use serde_json::json;

#[test]
fn test_input_output_schema_new_sets_fields() {
    let schema = InputOutputSchema::new(json!({"type": "object"}), "an object".to_string());
    assert_eq!(schema.description, "an object");
    assert!(schema.schema.is_object());
}

#[test]
fn test_input_output_schema_new_starts_with_no_examples() {
    let schema = InputOutputSchema::new(json!({}), "d".to_string());
    assert!(schema.examples.is_empty());
}

#[test]
fn test_input_output_schema_clone_preserves_schema() {
    let schema = InputOutputSchema::new(json!({"k": 1}), "d".to_string());
    let cloned = schema.clone();
    assert_eq!(cloned.schema, schema.schema);
}
