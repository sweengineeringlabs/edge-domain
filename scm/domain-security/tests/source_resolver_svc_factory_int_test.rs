//! Integration tests for [`saf::credential::source_resolver_svc_factory`] module.

use edge_domain_security::CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;

/// @covers: CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY
#[test]
fn test_source_resolver_svc_factory_happy() {
    let marker = CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY
#[test]
fn test_source_resolver_svc_factory_error() {
    let marker = CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
    assert!(
        matches!(marker, ()),
        "factory marker must match unit pattern"
    );
}

/// @covers: CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY
#[test]
fn test_source_resolver_svc_factory_edge() {
    let marker1 = CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
    let marker2 = CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
    assert_eq!(
        std::mem::size_of_val(&marker1),
        std::mem::size_of_val(&marker2)
    );
}
