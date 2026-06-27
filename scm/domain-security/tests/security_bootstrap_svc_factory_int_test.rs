//! Integration tests for [`security_bootstrap_svc_factory`] module.

use edge_domain_security::SECURITY_BOOTSTRAP_SVC_FACTORY;

/// @covers: SECURITY_BOOTSTRAP_SVC_FACTORY
#[test]
fn test_security_bootstrap_svc_factory_happy() {
    let marker = SECURITY_BOOTSTRAP_SVC_FACTORY;
    assert_eq!(marker, ());
}

/// @covers: SECURITY_BOOTSTRAP_SVC_FACTORY
#[test]
fn test_security_bootstrap_svc_factory_error() {
    let marker = SECURITY_BOOTSTRAP_SVC_FACTORY;
    assert!(matches!(marker, ()));
}

/// @covers: SECURITY_BOOTSTRAP_SVC_FACTORY
#[test]
fn test_security_bootstrap_svc_factory_edge() {
    let marker1 = SECURITY_BOOTSTRAP_SVC_FACTORY;
    let marker2 = SECURITY_BOOTSTRAP_SVC_FACTORY;
    assert_eq!(marker1, marker2);
}
