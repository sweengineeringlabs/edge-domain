//! Integration tests for `SagaError`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::SagaError;

/// @covers: SagaError
#[test]
fn test_saga_error_already_registered_display_contains_id() {
    let err = SagaError::AlreadyRegistered("order-1".to_string());
    assert!(err.to_string().contains("order-1"));
    assert!(err.to_string().contains("already registered"));
}

/// @covers: SagaError
#[test]
fn test_saga_error_not_found_display_contains_id() {
    let err = SagaError::NotFound("ghost".to_string());
    assert!(err.to_string().contains("ghost"));
    assert!(err.to_string().contains("no saga"));
}

/// @covers: SagaError
#[test]
fn test_saga_error_variants_are_distinct() {
    let a = SagaError::AlreadyRegistered("x".to_string());
    let b = SagaError::NotFound("x".to_string());
    assert_ne!(a, b);
}
