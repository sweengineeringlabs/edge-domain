//! Integration tests for [`SecurityContext`] type.

use edge_domain_security::SecurityBootstrap;

#[test]
fn test_security_context_unauthenticated_happy() {
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(
        !ctx.authenticated,
        "Unauthenticated context must have authenticated=false"
    );
}

#[test]
fn test_security_context_unauthenticated_error() {
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(
        ctx.principal.is_none(),
        "Unauthenticated context must have no principal"
    );
}

#[test]
fn test_security_context_unauthenticated_edge() {
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(
        ctx.claims.is_empty(),
        "Unauthenticated context must have no claims"
    );
}

#[test]
fn test_security_context_authenticated_happy() {
    let principal = Box::new(edge_domain_security::AnonymousPrincipal);
    let ctx =
        <edge_domain_security::SecurityServices as SecurityBootstrap>::authenticated(principal);
    assert!(
        ctx.authenticated,
        "Authenticated context must have authenticated=true"
    );
}

#[test]
fn test_security_context_authenticated_error() {
    let principal = Box::new(edge_domain_security::AnonymousPrincipal);
    let ctx =
        <edge_domain_security::SecurityServices as SecurityBootstrap>::authenticated(principal);
    assert!(
        ctx.principal.is_some(),
        "Authenticated context must have a principal"
    );
    let p = ctx.principal.expect("principal must be Some");
    assert_eq!(p.kind(), "anonymous", "Principal kind must be anonymous");
}

#[test]
fn test_security_context_authenticated_edge() {
    let principal = Box::new(edge_domain_security::AnonymousPrincipal);
    let ctx =
        <edge_domain_security::SecurityServices as SecurityBootstrap>::authenticated(principal);
    assert_eq!(ctx.is_authorized, false, "New context starts unauthorized");
}

#[test]
fn test_security_context_debug_happy() {
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let debug_str = format!("{:?}", ctx);
    assert!(!debug_str.is_empty(), "Debug output must not be empty");
    assert!(
        debug_str.contains("false"),
        "Debug must show authenticated state"
    );
}
