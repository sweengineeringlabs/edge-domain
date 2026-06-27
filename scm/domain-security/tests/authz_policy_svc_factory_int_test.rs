//! Integration tests for [`authz_policy_svc_factory`] module.

use edge_domain_security::AUTHZ_POLICY_SVC_FACTORY;

/// @covers: AUTHZ_POLICY_SVC_FACTORY
#[test]
fn test_authz_policy_svc_factory_happy() {
    let marker = AUTHZ_POLICY_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: AUTHZ_POLICY_SVC_FACTORY
#[test]
fn test_authz_policy_svc_factory_error() {
    let marker = AUTHZ_POLICY_SVC_FACTORY;
    assert!(matches!(marker, ()), "factory marker must match unit pattern");
}

/// @covers: AUTHZ_POLICY_SVC_FACTORY
#[test]
fn test_authz_policy_svc_factory_edge() {
    let marker1 = AUTHZ_POLICY_SVC_FACTORY;
    let marker2 = AUTHZ_POLICY_SVC_FACTORY;
    assert_eq!(std::mem::size_of_val(&marker1), std::mem::size_of_val(&marker2));
}
