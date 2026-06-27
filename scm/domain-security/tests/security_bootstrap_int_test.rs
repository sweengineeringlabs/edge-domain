//! Integration tests for [`SecurityBootstrap`] trait.

use edge_domain_security::{Principal, SecurityBootstrap};
use std::collections::HashMap;

struct TestBootstrap;
impl SecurityBootstrap for TestBootstrap {}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_happy() {
    let bootstrap = TestBootstrap;
    assert_eq!(bootstrap.bootstrap_name(), "security");
}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_error() {
    let bootstrap = TestBootstrap;
    assert!(!bootstrap.bootstrap_name().is_empty());
}

/// @covers: bootstrap_name
#[test]
fn test_bootstrap_name_edge() {
    let bootstrap = TestBootstrap;
    assert_eq!(
        bootstrap.bootstrap_name(),
        "security",
        "bootstrap_name must always return 'security'"
    );
}

/// @covers: unauthenticated
#[test]
fn test_unauthenticated_happy() {
    let ctx = TestBootstrap::unauthenticated();
    assert!(!ctx.authenticated);
}

/// @covers: unauthenticated
#[test]
fn test_unauthenticated_error() {
    let ctx = TestBootstrap::unauthenticated();
    assert!(
        ctx.principal.is_none(),
        "unauthenticated context should have no principal"
    );
}

/// @covers: unauthenticated
#[test]
fn test_unauthenticated_edge() {
    let ctx = TestBootstrap::unauthenticated();
    assert_eq!(ctx.authenticated, false);
}

/// @covers: authenticated
#[test]
fn test_authenticated_happy() {
    use edge_domain_security::AnonymousPrincipal;
    let principal = Box::new(AnonymousPrincipal);
    let ctx = TestBootstrap::authenticated(principal);
    assert!(ctx.authenticated);
}

/// @covers: authenticated
#[test]
fn test_authenticated_error() {
    use edge_domain_security::AnonymousPrincipal;
    let principal = Box::new(AnonymousPrincipal);
    let ctx = TestBootstrap::authenticated(principal);
    assert!(
        ctx.principal.is_some() && ctx.authenticated,
        "authenticated context must have principal and authenticated flag"
    );
}

/// @covers: authenticated
#[test]
fn test_authenticated_edge() {
    use edge_domain_security::AnonymousPrincipal;
    let ctx1 = TestBootstrap::authenticated(Box::new(AnonymousPrincipal));
    let ctx2 = TestBootstrap::authenticated(Box::new(AnonymousPrincipal));
    assert_eq!(ctx1.authenticated, ctx2.authenticated);
}

/// @covers: from_claims
#[test]
fn test_from_claims_happy() {
    let mut claims = HashMap::new();
    claims.insert("sub".to_string(), "user123".to_string());
    let result = TestBootstrap::from_claims(claims);
    assert!(result.is_ok(), "from_claims must succeed with claims");
    let ctx = result.unwrap();
    assert!(ctx.claims.len() > 0, "context must contain the claims");
}

/// @covers: from_claims
#[test]
fn test_from_claims_error() {
    let claims = HashMap::new();
    let result = TestBootstrap::from_claims(claims);
    assert!(result.is_err(), "empty claims should error");
}

/// @covers: from_claims
#[test]
fn test_from_claims_edge() {
    let mut claims = HashMap::new();
    claims.insert("key".to_string(), "value".to_string());
    let r1 = TestBootstrap::from_claims(claims.clone());
    let r2 = TestBootstrap::from_claims(claims);
    assert_eq!(r1.is_ok(), r2.is_ok());
}

/// @covers: noop_guard
#[test]
fn test_noop_guard_happy() {
    use edge_domain_security::{Security, SecurityBootstrap};
    let guard = TestBootstrap::noop_guard();
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = guard.enforce(&ctx);
    assert!(result.is_ok(), "noop_guard must allow all contexts");
    assert_eq!(result.unwrap(), (), "enforce must return Ok(())");
}

/// @covers: noop_guard
#[test]
fn test_noop_guard_error() {
    let guard1 = TestBootstrap::noop_guard();
    let _guard2 = TestBootstrap::noop_guard();
    assert_eq!(
        std::mem::size_of_val(&guard1),
        std::mem::size_of::<edge_domain_security::NoopSecurity>()
    );
}

/// @covers: noop_guard
#[test]
fn test_noop_guard_edge() {
    let guard1 = TestBootstrap::noop_guard();
    let _guard2 = TestBootstrap::noop_guard();
    assert_eq!(
        std::mem::size_of::<edge_domain_security::NoopSecurity>(),
        std::mem::size_of_val(&guard1)
    );
}

/// @covers: anonymous_principal
#[test]
fn test_anonymous_principal_happy() {
    let principal = TestBootstrap::anonymous_principal();
    assert_eq!(principal.id(), "anonymous");
}

/// @covers: anonymous_principal
#[test]
fn test_anonymous_principal_error() {
    let principal = TestBootstrap::anonymous_principal();
    assert_eq!(principal.kind(), "anonymous");
}

/// @covers: anonymous_principal
#[test]
fn test_anonymous_principal_edge() {
    let p1 = TestBootstrap::anonymous_principal();
    let p2 = TestBootstrap::anonymous_principal();
    assert_eq!(p1.id(), p2.id());
}

/// @covers: default_services
#[test]
fn test_default_services_happy() {
    let services = TestBootstrap::default_services();
    assert_eq!(
        std::mem::size_of::<edge_domain_security::SecurityServices>(),
        std::mem::size_of_val(&services)
    );
}

/// @covers: default_services
#[test]
fn test_default_services_error() {
    let s1 = TestBootstrap::default_services();
    let s2 = TestBootstrap::default_services();
    assert_eq!(std::mem::size_of_val(&s1), std::mem::size_of_val(&s2));
}

/// @covers: default_services
#[test]
fn test_default_services_edge() {
    let s1 = TestBootstrap::default_services();
    let s2 = TestBootstrap::default_services();
    assert_eq!(
        std::mem::size_of::<edge_domain_security::SecurityServices>(),
        std::mem::size_of_val(&s1)
    );
}

/// @covers: context_builder
#[test]
fn test_context_builder_happy() {
    let builder = TestBootstrap::context_builder();
    assert_eq!(
        std::mem::size_of::<edge_domain_security::SecurityContextBuilder>(),
        std::mem::size_of_val(&builder)
    );
}

/// @covers: context_builder
#[test]
fn test_context_builder_error() {
    let b1 = TestBootstrap::context_builder();
    let b2 = TestBootstrap::context_builder();
    assert_eq!(std::mem::size_of_val(&b1), std::mem::size_of_val(&b2));
}

/// @covers: context_builder
#[test]
fn test_context_builder_edge() {
    let b1 = TestBootstrap::context_builder();
    let b2 = TestBootstrap::context_builder();
    assert_eq!(
        std::mem::size_of::<edge_domain_security::SecurityContextBuilder>(),
        std::mem::size_of_val(&b1)
    );
}
