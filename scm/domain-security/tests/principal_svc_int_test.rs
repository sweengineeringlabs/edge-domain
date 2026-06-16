//! Integration tests — `Principal` and `AnonymousPrincipal` via SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, Principal};

/// @covers: Principal — AnonymousPrincipal is a valid Principal implementor
#[test]
fn test_anonymous_principal_implements_principal_trait_happy() {
    let _: &dyn Principal = &AnonymousPrincipal;
}

/// @covers: AnonymousPrincipal::ID — constant is non-empty
#[test]
fn test_anonymous_principal_id_constant_is_non_empty_error() {
    assert!(!AnonymousPrincipal::ID.is_empty());
}

/// @covers: AnonymousPrincipal::KIND — constant is non-empty
#[test]
fn test_anonymous_principal_kind_constant_is_non_empty_edge() {
    assert!(!AnonymousPrincipal::KIND.is_empty());
}
