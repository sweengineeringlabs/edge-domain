//! Integration tests — `NoopSecurity` guard.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, NoopSecurity, Security, SecurityContext};

/// @covers: NoopSecurity::enforce — accepts authenticated context
#[test]
fn test_enforce_authenticated_context_returns_ok_happy() {
    let ctx = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    assert_eq!(NoopSecurity.enforce(&ctx), Ok(()), "noop security should accept authenticated context");
}

/// @covers: NoopSecurity::enforce — accepts unauthenticated context too
#[test]
fn test_enforce_unauthenticated_context_returns_ok_error() {
    let ctx = SecurityContext::unauthenticated();
    assert_eq!(NoopSecurity.enforce(&ctx), Ok(()), "noop security should accept unauthenticated context");
}

/// @covers: NoopSecurity — zero-sized marker struct
#[test]
fn test_noop_security_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<NoopSecurity>(), 0);
}
