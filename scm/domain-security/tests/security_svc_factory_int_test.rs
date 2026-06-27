//! Integration tests for [`security_svc_factory`] module.

use edge_domain_security::SECURITY_SVC_FACTORY;

/// @covers: SECURITY_SVC_FACTORY
#[test]
fn test_security_svc_factory_happy() {
    let marker = SECURITY_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: SECURITY_SVC_FACTORY
#[test]
fn test_security_svc_factory_error() {
    let marker = SECURITY_SVC_FACTORY;
    assert!(
        matches!(marker, ()),
        "factory marker must match unit pattern"
    );
}

/// @covers: SECURITY_SVC_FACTORY
#[test]
fn test_security_svc_factory_edge() {
    let marker1 = SECURITY_SVC_FACTORY;
    let marker2 = SECURITY_SVC_FACTORY;
    assert_eq!(
        std::mem::size_of_val(&marker1),
        std::mem::size_of_val(&marker2)
    );
}
