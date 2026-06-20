//! Integration tests — `SecurityServices` zero-config factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{Security, SecurityBootstrap, SecurityServices};

/// @covers: SecurityServices — constructs a noop guard that enforces successfully
#[test]
fn test_security_services_noop_guard_enforces_ok_happy() {
    let guard = SecurityServices::noop_guard();
    let ctx = SecurityServices::unauthenticated();
    assert!(guard.enforce(&ctx).is_ok());
}

/// @covers: SecurityServices — is zero-sized (no configuration state)
#[test]
fn test_security_services_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<SecurityServices>(), 0);
}

/// @covers: SecurityServices — copy semantics yield equal instances
#[test]
fn test_security_services_copy_semantics_edge() {
    let a = SecurityServices;
    let b = a;
    assert_eq!(a, b);
}
