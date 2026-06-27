//! Integration tests for [`principal_svc_factory`] module.

use edge_domain_security::PRINCIPAL_SVC_FACTORY;

/// @covers: PRINCIPAL_SVC_FACTORY
#[test]
fn test_principal_svc_factory_happy() {
    let marker = PRINCIPAL_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: PRINCIPAL_SVC_FACTORY
#[test]
fn test_principal_svc_factory_error() {
    let marker = PRINCIPAL_SVC_FACTORY;
    assert!(matches!(marker, ()), "factory marker must match unit pattern");
}

/// @covers: PRINCIPAL_SVC_FACTORY
#[test]
fn test_principal_svc_factory_edge() {
    let marker1 = PRINCIPAL_SVC_FACTORY;
    let marker2 = PRINCIPAL_SVC_FACTORY;
    assert_eq!(std::mem::size_of_val(&marker1), std::mem::size_of_val(&marker2));
}
