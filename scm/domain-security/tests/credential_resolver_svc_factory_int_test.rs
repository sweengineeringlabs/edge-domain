//! Integration tests for [`credential_resolver_svc_factory`] module.

use edge_domain_security::{CREDENTIAL_RESOLVER_SVC_FACTORY, CREDENTIAL_RESOLVER_SVC};

/// @covers: CREDENTIAL_RESOLVER_SVC_FACTORY
#[test]
fn test_credential_resolver_svc_factory_happy() {
    let marker = CREDENTIAL_RESOLVER_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: CREDENTIAL_RESOLVER_SVC_FACTORY
#[test]
fn test_credential_resolver_svc_factory_error() {
    let marker = CREDENTIAL_RESOLVER_SVC_FACTORY;
    assert!(matches!(marker, ()), "factory marker must match unit pattern");
}

/// @covers: CREDENTIAL_RESOLVER_SVC_FACTORY
#[test]
fn test_credential_resolver_svc_factory_edge() {
    let marker1 = CREDENTIAL_RESOLVER_SVC_FACTORY;
    let marker2 = CREDENTIAL_RESOLVER_SVC_FACTORY;
    assert_eq!(marker1, marker2, "factory markers must be consistent");
}

/// @covers: CREDENTIAL_RESOLVER_SVC
#[test]
fn test_credential_resolver_svc_happy() {
    assert_eq!(CREDENTIAL_RESOLVER_SVC, "credential_resolver");
}

/// @covers: CREDENTIAL_RESOLVER_SVC
#[test]
fn test_credential_resolver_svc_error() {
    assert!(!CREDENTIAL_RESOLVER_SVC.is_empty(), "service identifier must not be empty");
}

/// @covers: CREDENTIAL_RESOLVER_SVC
#[test]
fn test_credential_resolver_svc_edge() {
    assert_eq!(CREDENTIAL_RESOLVER_SVC.len(), "credential_resolver".len(), "service identifier length must match");
}
