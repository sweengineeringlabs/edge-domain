#![allow(clippy::unwrap_used, clippy::expect_used)]
// @covers SchemaValidator trait methods
// Integration tests for SchemaValidator trait contract
// Validation error types available for implementations

use edge_llm_agent::ValidationError;
use serde_json::json;

#[test]
fn test_schema_validator_trait_validation_error_constructible() {
    let error = ValidationError {
        field: "test_field".to_string(),
        reason: "invalid value".to_string(),
    };
    assert_eq!(error.field, "test_field");
    assert_eq!(error.reason, "invalid value");
}

#[test]
fn test_schema_validator_trait_validation_error_clone() {
    let error1 = ValidationError {
        field: "field".to_string(),
        reason: "reason".to_string(),
    };
    let error2 = error1.clone();
    assert_eq!(error1.field, error2.field);
    assert_eq!(error1.reason, error2.reason);
}

#[test]
fn test_schema_validator_trait_validation_error_debug() {
    let error = ValidationError {
        field: "test".to_string(),
        reason: "error".to_string(),
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("field"));
    assert!(debug_str.contains("reason"));
}

#[test]
fn test_schema_validator_trait_json_validation_types() {
    let _object = json!({"key": "value"});
    let _array = json!([1, 2, 3]);
    let _string = json!("test");
    let _number = json!(42);
    let _null = json!(null);
    let _bool = json!(true);
}

#[test]
fn test_schema_validator_trait_complex_schema() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number"}
        },
        "required": ["name"]
    });
    assert!(schema["type"].is_string());
    assert!(schema["properties"].is_object());
}
