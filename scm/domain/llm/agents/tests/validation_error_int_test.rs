#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `ValidationError` value type.

use edge_llm_agent::ValidationError;

#[test]
fn test_validation_error_new_sets_fields() {
    let err = ValidationError::new("field".to_string(), "reason".to_string());
    assert_eq!(err.field, "field");
    assert_eq!(err.reason, "reason");
}

#[test]
fn test_validation_error_clone_preserves_fields() {
    let err = ValidationError::new("f".to_string(), "r".to_string());
    let cloned = err.clone();
    assert_eq!(cloned.field, err.field);
    assert_eq!(cloned.reason, err.reason);
}

#[test]
fn test_validation_error_debug_contains_fields() {
    let err = ValidationError::new("amount".to_string(), "too small".to_string());
    let debug = format!("{:?}", err);
    assert!(debug.contains("amount"));
    assert!(debug.contains("too small"));
}
