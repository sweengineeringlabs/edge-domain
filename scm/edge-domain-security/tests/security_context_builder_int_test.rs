//! Integration tests — `SecurityContextBuilder` fluent builder.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, SecurityContextBuilder};

/// @covers: SecurityContextBuilder::build — empty builder yields unauthenticated context
#[test]
fn test_build_empty_builder_unauthenticated_happy() {
    let ctx = SecurityContextBuilder::new().build();
    assert!(!ctx.authenticated);
    assert!(ctx.principal.is_none());
    assert!(ctx.claims.is_empty());
}

/// @covers: SecurityContextBuilder::build — unauthenticated has no tenant or trace
#[test]
fn test_build_unauthenticated_no_tenant_no_trace_error() {
    let ctx = SecurityContextBuilder::new().build();
    assert!(ctx.tenant_id.is_none());
    assert!(ctx.trace_id.is_none());
}

/// @covers: SecurityContextBuilder::build — chained builder sets all fields
#[test]
fn test_build_chained_builder_sets_all_fields_edge() {
    let ctx = SecurityContextBuilder::new()
        .principal(Box::new(AnonymousPrincipal))
        .tenant_id("acme")
        .trace_id("trace-1")
        .claim("role", "admin")
        .build();
    assert!(ctx.authenticated);
    assert!(ctx.principal.is_some());
    assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
    assert_eq!(ctx.trace_id.as_deref(), Some("trace-1"));
    assert_eq!(ctx.claim("role"), Some("admin"));
}

/// @covers: SecurityContextBuilder::default — same as new()
#[test]
fn test_default_equals_new_happy() {
    let a = SecurityContextBuilder::new().build();
    let b = SecurityContextBuilder::default().build();
    assert_eq!(a.authenticated, b.authenticated);
    assert!(b.principal.is_none());
}

/// @covers: SecurityContextBuilder::claim — missing key returns None after build
#[test]
fn test_claim_missing_after_build_returns_none_error() {
    let ctx = SecurityContextBuilder::new().claim("foo", "bar").build();
    assert!(ctx.claim("missing").is_none());
}

/// @covers: SecurityContextBuilder — two independent builds are separate instances
#[test]
fn test_two_builds_are_independent_edge() {
    let a = SecurityContextBuilder::new().claim("k", "v1").build();
    let b = SecurityContextBuilder::new().claim("k", "v2").build();
    assert_ne!(a.claim("k"), b.claim("k"));
}
