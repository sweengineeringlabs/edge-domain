//! Integration tests — `AnonymousPrincipal` struct.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::AnonymousPrincipal;

/// @covers: AnonymousPrincipal — constructs without arguments
#[test]
fn test_construct_anonymous_principal_succeeds_happy() {
    let _ = AnonymousPrincipal;
}

/// @covers: AnonymousPrincipal — size is zero (marker struct, no heap alloc)
#[test]
fn test_anonymous_principal_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<AnonymousPrincipal>(), 0);
}

/// @covers: AnonymousPrincipal — clone produces equal value
#[test]
fn test_clone_is_equal_to_original_edge() {
    let a = AnonymousPrincipal;
    let b = a;
    assert_eq!(a, b);
}
