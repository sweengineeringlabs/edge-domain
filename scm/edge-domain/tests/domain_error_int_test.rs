//! Integration tests — `DomainError` type.

use edge_application::DomainError;

/// @covers: DomainError::Unavailable — Display formats the inner message
#[test]
fn test_unavailable_error_formats_message_happy() {
    let e = DomainError::Unavailable("component-x".into());
    assert!(e.to_string().contains("component-x"));
}

/// @covers: DomainError::ExtensionRejected — Display formats the inner message
#[test]
fn test_extension_rejected_error_formats_message_error() {
    let e = DomainError::ExtensionRejected("hook-a rejected".into());
    assert!(e.to_string().contains("hook-a rejected"));
}

/// @covers: DomainError — Debug output is non-empty
#[test]
fn test_domain_error_is_debug_edge() {
    let e = DomainError::Unavailable("x".into());
    assert!(!format!("{e:?}").is_empty());
}
