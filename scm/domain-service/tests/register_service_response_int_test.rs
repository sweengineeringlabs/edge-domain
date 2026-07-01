//! Tests for [`RegisterServiceResponse`] — zero-sized marker type.

use std::mem::size_of;
use edge_domain_service::RegisterServiceResponse;

/// @covers: RegisterServiceResponse — constructible
#[test]
fn test_register_service_response_constructible_happy() {
    assert_eq!(size_of::<RegisterServiceResponse>(), 0);
}

/// @covers: RegisterServiceResponse — multiple instances identical
#[test]
fn test_register_service_response_multiple_instances_identical_edge() {
    let a = RegisterServiceResponse;
    let b = RegisterServiceResponse;
    assert_eq!(a, b);
}
