//! Integration tests for Principal service factory.

use edge_domain_security::PRINCIPAL_SVC_FACTORY;

#[test]
fn test_principal_svc_factory_exists() {
    assert_eq!(PRINCIPAL_SVC_FACTORY, ());
}
