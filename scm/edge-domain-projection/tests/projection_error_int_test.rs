//! Integration tests for `ProjectionError`.

use edge_domain_projection::ProjectionError;

#[test]
fn test_empty_stream_display_contains_empty_happy() {
    let msg = ProjectionError::EmptyStream.to_string();
    assert!(msg.contains("empty"), "expected 'empty' in: {msg}");
}

#[test]
fn test_internal_error_display_contains_message_error() {
    let msg = ProjectionError::Internal("bad state".into()).to_string();
    assert!(msg.contains("bad state"), "expected 'bad state' in: {msg}");
}

#[test]
fn test_errors_are_equality_comparable_edge() {
    assert_eq!(ProjectionError::EmptyStream, ProjectionError::EmptyStream);
    assert_eq!(
        ProjectionError::Internal("x".into()),
        ProjectionError::Internal("x".into())
    );
    assert_ne!(
        ProjectionError::EmptyStream,
        ProjectionError::Internal("x".into())
    );
}
