//! Integration tests for [`SecretString`] type.

use edge_domain_security::SecretString;

#[test]
fn test_secret_string_from_str_happy() {
    let secret = SecretString::from("test");
    assert_eq!(secret, SecretString::from("test"), "SecretString must preserve value");
}

#[test]
fn test_secret_string_from_string_happy() {
    let secret = SecretString::from("test".to_string());
    assert_eq!(secret, SecretString::from("test"), "SecretString must handle owned strings");
}

#[test]
fn test_secret_string_clone_happy() {
    let secret = SecretString::from("test");
    let cloned = secret.clone();
    assert_eq!(secret, cloned, "Cloned secret must equal original");
}

#[test]
fn test_secret_string_debug_redaction_happy() {
    let secret = SecretString::from("sensitive");
    let debug_str = format!("{:?}", secret);
    assert!(!debug_str.contains("sensitive"), "Debug output must not expose secret");
    assert!(debug_str.contains("REDACTED"), "Debug output must indicate redaction");
}

#[test]
fn test_secret_string_display_redaction_happy() {
    let secret = SecretString::from("sensitive");
    let display_str = format!("{}", secret);
    assert!(!display_str.contains("sensitive"), "Display output must not expose secret");
}

#[test]
fn test_secret_string_equality_edge() {
    let s1 = SecretString::from("x");
    let s2 = SecretString::from("x");
    assert_eq!(s1, s2, "Identical secrets must be equal");
}
