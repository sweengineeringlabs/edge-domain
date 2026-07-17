//! Tests for [`EmptinessRequest`] — zero-sized marker type.

use edge_application_service::EmptinessRequest;
use std::mem::size_of;

/// @covers: EmptinessRequest — constructible
#[test]
fn test_emptiness_request_constructible_happy() {
    assert_eq!(size_of::<EmptinessRequest>(), 0);
}

/// @covers: EmptinessRequest — multiple instances identical
#[test]
fn test_emptiness_request_multiple_instances_identical_edge() {
    let a = EmptinessRequest;
    let b = EmptinessRequest;
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}
