//! Tests for [`NameRequest`] — zero-sized marker type.

use edge_application_service::NameRequest;
use std::mem::size_of;

/// @covers: NameRequest — constructible
#[test]
fn test_name_request_constructible_happy() {
    let _ = NameRequest;
    assert_eq!(size_of::<NameRequest>(), 0);
}

/// @covers: NameRequest — copy semantics
#[test]
fn test_name_request_copy_semantics_happy() {
    let a = NameRequest;
    let b = a;
    let c = a;
    assert_eq!((a, b), (c, c));
}

/// @covers: NameRequest — multiple instances are identical
#[test]
fn test_name_request_multiple_instances_identical_edge() {
    let a = NameRequest;
    let b = NameRequest;
    assert_eq!(a, b);
}
