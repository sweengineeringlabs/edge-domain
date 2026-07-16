//! Tests for [`NoopRequest`] — zero-sized payload type.

use edge_application_service::NoopRequest;
use std::mem::size_of;

/// @covers: NoopRequest — constructible
#[test]
fn test_noop_request_constructible_happy() {
    assert_eq!(size_of::<NoopRequest>(), 0);
}

/// @covers: NoopRequest — multiple instances identical
#[test]
fn test_noop_request_multiple_instances_identical_edge() {
    let a = NoopRequest;
    let b = NoopRequest;
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

