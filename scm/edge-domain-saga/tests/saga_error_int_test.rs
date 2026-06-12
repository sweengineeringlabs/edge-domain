//! Integration tests for `SagaError`.

use edge_domain_saga::SagaError;

#[test]
fn test_already_registered_display_contains_id_happy() {
    let msg = SagaError::AlreadyRegistered("proc-1".into()).to_string();
    assert!(msg.contains("proc-1"), "expected 'proc-1' in: {msg}");
}

#[test]
fn test_not_found_display_contains_id_error() {
    let msg = SagaError::NotFound("missing".into()).to_string();
    assert!(msg.contains("missing"), "expected 'missing' in: {msg}");
}

#[test]
fn test_errors_are_equality_comparable_edge() {
    assert_eq!(
        SagaError::AlreadyRegistered("x".into()),
        SagaError::AlreadyRegistered("x".into())
    );
    assert_eq!(
        SagaError::NotFound("y".into()),
        SagaError::NotFound("y".into())
    );
    assert_ne!(
        SagaError::AlreadyRegistered("x".into()),
        SagaError::NotFound("x".into())
    );
}
