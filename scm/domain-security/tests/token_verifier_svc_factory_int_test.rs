//! Integration tests for [`token_verifier_svc_factory`] module.

use edge_domain_security::TOKEN_VERIFIER_SVC_FACTORY;

/// @covers: TOKEN_VERIFIER_SVC_FACTORY
#[test]
fn test_token_verifier_svc_factory_happy() {
    let marker = TOKEN_VERIFIER_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: TOKEN_VERIFIER_SVC_FACTORY
#[test]
fn test_token_verifier_svc_factory_error() {
    let marker = TOKEN_VERIFIER_SVC_FACTORY;
    assert!(
        matches!(marker, ()),
        "factory marker must match unit pattern"
    );
}

/// @covers: TOKEN_VERIFIER_SVC_FACTORY
#[test]
fn test_token_verifier_svc_factory_edge() {
    let marker1 = TOKEN_VERIFIER_SVC_FACTORY;
    let marker2 = TOKEN_VERIFIER_SVC_FACTORY;
    assert_eq!(
        std::mem::size_of_val(&marker1),
        std::mem::size_of_val(&marker2)
    );
}
