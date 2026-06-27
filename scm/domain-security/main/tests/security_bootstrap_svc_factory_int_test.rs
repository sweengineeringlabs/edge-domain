//! Integration tests for SecurityBootstrap service factory.

use edge_domain_security::SECURITY_BOOTSTRAP_SVC_FACTORY;

#[test]
fn test_security_bootstrap_svc_factory_exists() {
    assert_eq!(SECURITY_BOOTSTRAP_SVC_FACTORY, ());
}
