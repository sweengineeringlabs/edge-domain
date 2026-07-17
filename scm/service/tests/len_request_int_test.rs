//! Tests for [`LenRequest`] — zero-sized marker type.

use edge_application_service::LenRequest;
use std::mem::size_of;

/// @covers: LenRequest — constructible
#[test]
fn test_len_request_constructible_happy() {
    assert_eq!(size_of::<LenRequest>(), 0);
}

/// @covers: LenRequest — multiple instances identical
#[test]
fn test_len_request_multiple_instances_identical_edge() {
    let a = LenRequest;
    let b = LenRequest;
    assert_eq!(a, b);
}
