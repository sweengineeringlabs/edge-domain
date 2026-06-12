//! Integration tests — `RequestContextBuilder` type.

use std::collections::HashMap;

use edge_domain_handler::RequestContextBuilder;

/// @covers: RequestContextBuilder::build — defaults produce unauthenticated context
#[test]
fn test_build_defaults_produce_unauthenticated_context_happy() {
    let ctx = RequestContextBuilder::new().build();
    assert!(!ctx.authenticated);
    assert!(ctx.subject.is_none());
}

/// @covers: RequestContextBuilder — full builder chain
#[test]
fn test_build_full_chain_sets_all_fields_happy() {
    let mut claims = HashMap::new();
    claims.insert("scope".into(), "read".into());
    let ctx = RequestContextBuilder::new()
        .with_subject("carol")
        .with_issuer("auth.example")
        .with_tenant_id("t-99")
        .with_trace_id("trace-abc")
        .authenticated()
        .with_claims(claims)
        .build();
    assert!(ctx.authenticated);
    assert_eq!(ctx.subject.as_deref(), Some("carol"));
    assert_eq!(ctx.issuer.as_deref(), Some("auth.example"));
    assert_eq!(ctx.tenant_id.as_deref(), Some("t-99"));
    assert_eq!(ctx.trace_id, "trace-abc");
    assert_eq!(ctx.claims.get("scope").map(String::as_str), Some("read"));
}

/// @covers: RequestContextBuilder — authenticated flag
#[test]
fn test_build_authenticated_flag_sets_authenticated_happy() {
    let ctx = RequestContextBuilder::new().authenticated().build();
    assert!(ctx.authenticated);
}

/// @covers: RequestContextBuilder — without trace_id uses empty string
#[test]
fn test_build_without_trace_id_uses_empty_string_edge() {
    let ctx = RequestContextBuilder::new().build();
    assert!(ctx.trace_id.is_empty());
}

/// @covers: RequestContextBuilder — multiple with_claims calls merge
#[test]
fn test_build_multiple_with_claims_calls_merge_edge() {
    let mut c1 = HashMap::new();
    c1.insert("a".into(), "1".into());
    let mut c2 = HashMap::new();
    c2.insert("b".into(), "2".into());
    let ctx = RequestContextBuilder::new()
        .with_claims(c1)
        .with_claims(c2)
        .build();
    assert_eq!(ctx.claims.len(), 2);
    assert_eq!(ctx.claims.get("a").map(String::as_str), Some("1"));
    assert_eq!(ctx.claims.get("b").map(String::as_str), Some("2"));
}

/// @covers: RequestContextBuilder::new — is same as Default
#[test]
fn test_new_and_default_produce_equivalent_builders_edge() {
    let ctx_new = RequestContextBuilder::new().build();
    let ctx_default = RequestContextBuilder::default().build();
    assert_eq!(ctx_new.authenticated, ctx_default.authenticated);
    assert_eq!(ctx_new.subject, ctx_default.subject);
}
