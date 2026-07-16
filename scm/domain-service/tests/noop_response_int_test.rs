//! Tests for [`NoopResponse`] — zero-sized payload type.

use edge_application_service::NoopResponse;
use std::mem::size_of;

/// @covers: NoopResponse — constructible
#[test]
fn test_noop_response_constructible_happy() {
    assert_eq!(size_of::<NoopResponse>(), 0);
}

/// @covers: NoopResponse — multiple instances identical
#[test]
fn test_noop_response_multiple_instances_identical_edge() {
    let a = NoopResponse;
    let b = NoopResponse;
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

