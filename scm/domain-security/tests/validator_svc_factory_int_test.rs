//! Integration tests for Validator svc factory.

/// @covers: VALIDATOR_SVC_FACTORY
#[test]
fn test_validator_svc_factory_exists() {
    let _ = edge_domain_security::VALIDATOR_SVC_FACTORY;
}
