//! Integration tests for [`ValidationError`] type.

use edge_domain_security::ValidationError;

#[test]
fn test_validation_error_creation_happy() {
    let error = ValidationError("test".to_string());
    assert!(!error.0.is_empty(), "ValidationError must contain message");
}

#[test]
fn test_validation_error_creation_error() {
    let error = ValidationError("".to_string());
    assert_eq!(error.0, "", "ValidationError must preserve empty message");
}

#[test]
fn test_validation_error_creation_edge() {
    let e1 = ValidationError("test".to_string());
    let e2 = ValidationError("test".to_string());
    assert_eq!(e1, e2, "Identical validation errors must be equal");
}

#[test]
fn test_validation_error_clone_happy() {
    let error = ValidationError("test".to_string());
    let cloned = error.clone();
    assert_eq!(error, cloned, "Cloned error must equal original");
}

#[test]
fn test_validation_error_debug_happy() {
    let error = ValidationError("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
}

#[test]
fn test_validation_error_display_happy() {
    let error = ValidationError("test message".to_string());
    let display_str = format!("{}", error);
    assert!(
        display_str.contains("test message"),
        "Display must contain message"
    );
}
