//! Tests for the `ValidatorError` error type.

use edge_domain_validator::ValidatorError;

/// @covers: ValidatorError::Invalid — Display includes message
#[test]
fn test_invalid_display_includes_message_happy() {
    let e = ValidatorError::Invalid("bad range".into());
    assert!(e.to_string().contains("bad range"));
}

/// @covers: ValidatorError::Invalid — equality by value
#[test]
fn test_invalid_equality_by_value_error() {
    assert_eq!(
        ValidatorError::Invalid("x".into()),
        ValidatorError::Invalid("x".into())
    );
}

/// @covers: ValidatorError::Invalid — distinct messages differ
#[test]
fn test_invalid_distinct_messages_differ_edge() {
    assert_ne!(
        ValidatorError::Invalid("a".into()),
        ValidatorError::Invalid("b".into())
    );
}
