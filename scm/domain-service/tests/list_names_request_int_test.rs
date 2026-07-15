//! Tests for [`ListNamesRequest`] — zero-sized marker type.

use edge_application_service::ListNamesRequest;
use std::mem::size_of;

/// @covers: ListNamesRequest — constructible
#[test]
fn test_list_names_request_constructible_happy() {
    assert_eq!(size_of::<ListNamesRequest>(), 0);
}

/// @covers: ListNamesRequest — multiple instances identical
#[test]
fn test_list_names_request_multiple_instances_identical_edge() {
    let a = ListNamesRequest;
    let b = ListNamesRequest;
    assert_eq!(a, b);
}
