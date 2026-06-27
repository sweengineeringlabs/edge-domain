//! Integration tests for Security service factory.

use edge_domain_security::SECURITY_SVC_FACTORY;

#[test]
fn test_security_svc_factory_exists() {
    assert_eq!(SECURITY_SVC_FACTORY, ());
}
