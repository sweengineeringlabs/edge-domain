//! Integration tests for [`SecurityContextBuilder`] type.
//!
//! Note: Most builder methods are `pub(crate)` and only accessible via
//! [`SecurityBootstrap`](crate::SecurityBootstrap) trait. This test file
//! provides coverage for the public type definition.

use edge_domain_security::SecurityBootstrap;

#[test]
fn test_builder_via_bootstrap_happy() {
    // SecurityContextBuilder is constructed and used through SecurityBootstrap
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert!(
        !ctx.authenticated,
        "builder must create unauthenticated context"
    );
}

#[test]
fn test_builder_via_bootstrap_edge() {
    let ctx1 = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let ctx2 = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    assert_eq!(
        ctx1.authenticated, ctx2.authenticated,
        "multiple builders must produce consistent state"
    );
}
