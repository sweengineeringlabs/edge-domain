//! Integration tests — `RequestContext` type.

use std::collections::HashMap;

use edge_domain_handler::RequestContext;

/// @covers: RequestContext::unauthenticated — fields are unset
#[test]
fn test_unauthenticated_all_optional_fields_are_none_happy() {
    let ctx = RequestContext::unauthenticated();
    assert!(!ctx.authenticated);
    assert!(ctx.subject.is_none());
    assert!(ctx.issuer.is_none());
    assert!(ctx.tenant_id.is_none());
    assert!(ctx.trace_id.is_empty());
    assert!(ctx.claims.is_empty());
}

/// @covers: RequestContext::authenticated — sets fields correctly
#[test]
fn test_authenticated_sets_subject_issuer_tenant_claims_happy() {
    let mut claims = HashMap::new();
    claims.insert("role".into(), "admin".into());
    let ctx = RequestContext::authenticated(
        "alice",
        Some("issuer.example".into()),
        Some("tenant-42".into()),
        claims,
    );
    assert!(ctx.authenticated);
    assert_eq!(ctx.subject.as_deref(), Some("alice"));
    assert_eq!(ctx.issuer.as_deref(), Some("issuer.example"));
    assert_eq!(ctx.tenant_id.as_deref(), Some("tenant-42"));
    assert_eq!(ctx.claims.get("role").map(String::as_str), Some("admin"));
}

/// @covers: RequestContext::require_subject — unauthenticated returns None
#[test]
fn test_require_subject_unauthenticated_returns_none_error() {
    let ctx = RequestContext::unauthenticated();
    assert!(ctx.require_subject().is_none());
}

/// @covers: RequestContext::require_subject — authenticated returns subject
#[test]
fn test_require_subject_authenticated_returns_subject_happy() {
    let ctx = RequestContext::authenticated("bob", None, None, HashMap::new());
    assert_eq!(ctx.require_subject(), Some("bob"));
}

/// @covers: RequestContext::with_trace_id
#[test]
fn test_with_trace_id_sets_trace_id_edge() {
    let ctx = RequestContext::unauthenticated().with_trace_id("t-001");
    assert_eq!(ctx.trace_id, "t-001");
}

/// @covers: RequestContext::with_tenant_id
#[test]
fn test_with_tenant_id_sets_tenant_id_edge() {
    let ctx = RequestContext::unauthenticated().with_tenant_id("tenant-007");
    assert_eq!(ctx.tenant_id.as_deref(), Some("tenant-007"));
}

/// @covers: RequestContext default derive
#[test]
fn test_default_is_unauthenticated_edge() {
    let ctx = RequestContext::default();
    assert!(!ctx.authenticated);
}
