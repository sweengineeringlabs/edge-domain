//! Integration tests for SAF security bootstrap service facade module.
//! This test file provides coverage for main/src/saf/security/bootstrap/svc.rs
//! which re-exports the SecurityBootstrap trait.

use edge_domain_security::{SecurityBootstrap, SecurityServices};

#[test]
fn test_bootstrap_facade_trait_available_happy() {
    // Verify that the trait is accessible through the public API
    let _: &dyn SecurityBootstrap = &SecurityServices;
}

#[test]
fn test_bootstrap_facade_unauthenticated_edge() {
    // Verify bootstrap methods are callable through the facade
    let ctx = <SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(
        !ctx.authenticated,
        "default context must be unauthenticated"
    );
}

#[test]
fn test_bootstrap_facade_authenticated_edge() {
    let principal = Box::new(edge_domain_security::AnonymousPrincipal);
    let ctx = <SecurityServices as SecurityBootstrap>::authenticated(principal);
    assert!(
        ctx.authenticated,
        "authenticated context must be authenticated"
    );
}
