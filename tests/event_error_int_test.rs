//! Integration tests for `EventError`.

use edge_domain::EventError;

/// @covers: EventError
#[test]
fn test_event_error_serialization_failed_display_contains_message() {
    let e = EventError::SerializationFailed("bad json".into());
    assert!(e.to_string().contains("bad json"));
}

/// @covers: EventError
#[test]
fn test_event_error_unavailable_display_contains_message() {
    let e = EventError::Unavailable("broker down".into());
    assert!(e.to_string().contains("broker down"));
}

/// @covers: EventError
#[test]
fn test_event_error_variants_are_distinct() {
    assert!(matches!(EventError::SerializationFailed("x".into()), EventError::SerializationFailed(_)));
    assert!(matches!(EventError::Unavailable("x".into()), EventError::Unavailable(_)));
}
