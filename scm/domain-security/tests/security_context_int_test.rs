//! Integration tests — `SecurityContext` carrier struct.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::{AnonymousPrincipal, SecurityContext};

/// @covers: SecurityContext::unauthenticated — authenticated flag is false
#[test]
fn test_unauthenticated_returns_false_flag_happy() {
    let ctx = SecurityContext::unauthenticated();
    assert!(!ctx.authenticated);
}

/// @covers: SecurityContext::authenticated_with — authenticated flag is true
#[test]
fn test_authenticated_with_sets_true_flag_happy() {
    let ctx = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    assert!(ctx.authenticated);
}

/// @covers: SecurityContext::unauthenticated — claims are empty
#[test]
fn test_unauthenticated_has_empty_claims_error() {
    let ctx = SecurityContext::unauthenticated();
    assert!(ctx.claims.is_empty());
}

/// @covers: SecurityContext::with_claim — stores and retrieves value
#[test]
fn test_with_claim_stores_and_retrieves_value_happy() {
    let ctx = SecurityContext::unauthenticated()
        .with_claim("role", "admin");
    assert_eq!(ctx.claim("role"), Some("admin"));
}

/// @covers: SecurityContext::claim — missing key returns None
#[test]
fn test_claim_missing_key_returns_none_error() {
    let ctx = SecurityContext::unauthenticated();
    assert!(ctx.claim("role").is_none());
}

/// @covers: SecurityContext::with_tenant / with_trace_id — chaining works
#[test]
fn test_builder_chain_sets_all_fields_edge() {
    let ctx = SecurityContext::unauthenticated()
        .with_tenant("acme")
        .with_trace_id("trace-123")
        .with_claim("sub", "user-1");
    assert_eq!(ctx.tenant_id.as_deref(), Some("acme"));
    assert_eq!(ctx.trace_id.as_deref(), Some("trace-123"));
    assert_eq!(ctx.claim("sub"), Some("user-1"));
}

/// @covers: SecurityContext — no principal on unauthenticated
#[test]
fn test_unauthenticated_principal_is_none_edge() {
    let ctx = SecurityContext::unauthenticated();
    assert!(ctx.principal.is_none());
}
