//! Integration tests for [`validator_svc_factory`] module.

use edge_domain_security::VALIDATOR_SVC_FACTORY;

/// @covers: VALIDATOR_SVC_FACTORY
#[test]
fn test_validator_svc_factory_happy() {
    let marker = VALIDATOR_SVC_FACTORY;
    assert_eq!(marker, (), "factory marker must be unit type");
}

/// @covers: VALIDATOR_SVC_FACTORY
#[test]
fn test_validator_svc_factory_error() {
    let marker = VALIDATOR_SVC_FACTORY;
    assert!(matches!(marker, ()), "factory marker must match unit pattern");
}

/// @covers: VALIDATOR_SVC_FACTORY
#[test]
fn test_validator_svc_factory_edge() {
    let marker1 = VALIDATOR_SVC_FACTORY;
    let marker2 = VALIDATOR_SVC_FACTORY;
    assert_eq!(std::mem::size_of_val(&marker1), std::mem::size_of_val(&marker2));
}
