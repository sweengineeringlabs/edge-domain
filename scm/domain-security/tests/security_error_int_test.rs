//! Integration tests for [`SecurityError`] type.

use edge_domain_security::SecurityError;

#[test]
fn test_security_error_auth_variant_happy() {
    let error = SecurityError::Auth("test".to_string());
    assert!(
        matches!(error, SecurityError::Auth(_)),
        "Auth variant must match"
    );
}

#[test]
fn test_security_error_auth_variant_error() {
    let error = SecurityError::Auth("".to_string());
    assert_eq!(
        error,
        SecurityError::Auth("".to_string()),
        "Empty auth error must match"
    );
}

#[test]
fn test_security_error_auth_variant_edge() {
    let e1 = SecurityError::Auth("x".to_string());
    let e2 = SecurityError::Auth("x".to_string());
    assert_eq!(e1, e2, "Identical auth errors must be equal");
}

#[test]
fn test_security_error_clone_happy() {
    let error = SecurityError::Auth("test".to_string());
    let cloned = error.clone();
    assert_eq!(error, cloned, "Cloned error must equal original");
}

#[test]
fn test_security_error_debug_happy() {
    let error = SecurityError::Auth("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
}

#[test]
fn test_security_error_display_happy() {
    let error = SecurityError::Auth("test".to_string());
    let display_str = format!("{}", error);
    assert!(
        display_str.contains("auth"),
        "Display must contain variant info"
    );
}
