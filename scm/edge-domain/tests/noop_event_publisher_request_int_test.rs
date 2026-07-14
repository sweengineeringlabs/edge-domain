//! Integration tests for `NoopEventPublisherRequest`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::NoopEventPublisherRequest;

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_default_happy() {
    assert_eq!(
        NoopEventPublisherRequest::default(),
        NoopEventPublisherRequest
    );
}

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_debug_format_error() {
    assert_eq!(
        format!("{NoopEventPublisherRequest:?}"),
        "NoopEventPublisherRequest"
    );
}

/// @covers: NoopEventPublisherRequest
#[test]
fn test_noop_event_publisher_request_is_copy_edge() {
    let a = NoopEventPublisherRequest;
    let b = a;
    assert_eq!(a, b);
}
