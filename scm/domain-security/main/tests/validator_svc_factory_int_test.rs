//! Integration tests for Validator service factory.

use edge_domain_security::VALIDATOR_SVC_FACTORY;

#[test]
fn test_validator_svc_factory_exists() {
    assert_eq!(VALIDATOR_SVC_FACTORY, ());
}
