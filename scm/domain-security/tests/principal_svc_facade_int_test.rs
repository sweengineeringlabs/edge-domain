//! Integration tests for SAF principal service facade module.
//! This test file provides coverage for main/src/saf/security/principal/svc.rs
//! which re-exports the Principal trait.

use edge_domain_security::{AnonymousPrincipal, Principal};

#[test]
fn test_principal_facade_trait_available_happy() {
    // Verify that the trait is accessible through the public API
    let principal: &dyn Principal = &AnonymousPrincipal;
    let id = principal.id();
    assert_eq!(id, "anonymous");
}

#[test]
fn test_principal_facade_kind_edge() {
    // Verify principal methods are callable through the facade
    let principal: &dyn Principal = &AnonymousPrincipal;
    let kind = principal.kind();
    assert_eq!(kind, "anonymous");
}

#[test]
fn test_principal_facade_consistent_edge() {
    // Verify trait methods return consistent results
    let principal: &dyn Principal = &AnonymousPrincipal;
    let id1 = principal.id();
    let id2 = principal.id();
    assert_eq!(id1, id2, "Principal id must be consistent");
}
