//! Integration tests — `ValueObjectError` variants and display.
#![cfg(feature = "valueobject")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::ValueObjectError;

/// @covers: ValueObjectError — Empty display is non-empty
#[test]
fn test_empty_display_describes_constraint_happy() {
    let msg = ValueObjectError::Empty.to_string();
    assert!(!msg.is_empty());
}

/// @covers: ValueObjectError — Invalid display contains supplied message
#[test]
fn test_invalid_display_contains_supplied_message_happy() {
    let msg = ValueObjectError::Invalid("bad format".to_string()).to_string();
    assert!(msg.contains("bad format"), "got: {msg}");
}

/// @covers: ValueObjectError — distinct variants are not equal
#[test]
fn test_empty_and_invalid_are_not_equal_error() {
    assert_ne!(
        ValueObjectError::Empty,
        ValueObjectError::Invalid("x".to_string())
    );
}

/// @covers: ValueObjectError — clone preserves equality
#[test]
fn test_clone_preserves_equality_edge() {
    let e = ValueObjectError::Invalid("oops".to_string());
    assert_eq!(e.clone(), e);
}
