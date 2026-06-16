//! Integration tests for `RegistryError` — covers the errors/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::RegistryError;

/// @covers: RegistryError::DuplicateId — display message includes the id
#[test]
fn test_duplicate_id_display_includes_id_happy() {
    let e = RegistryError::DuplicateId("svc".to_string());
    assert!(e.to_string().contains("svc"));
    assert!(e.to_string().contains("already registered"));
}

/// @covers: RegistryError::DuplicateId — equality discriminates on id
#[test]
fn test_duplicate_id_equality_discriminates_error() {
    assert_eq!(
        RegistryError::DuplicateId("a".to_string()),
        RegistryError::DuplicateId("a".to_string())
    );
    assert_ne!(
        RegistryError::DuplicateId("a".to_string()),
        RegistryError::DuplicateId("b".to_string())
    );
}

/// @covers: RegistryError::DuplicateId — empty id still renders a message
#[test]
fn test_duplicate_id_empty_id_renders_edge() {
    let e = RegistryError::DuplicateId(String::new());
    assert!(e.to_string().contains("already registered"));
}
