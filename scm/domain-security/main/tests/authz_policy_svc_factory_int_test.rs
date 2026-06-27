//! Integration tests for AuthzPolicy service factory.

use edge_domain_security::AUTHZ_POLICY_SVC_FACTORY;

#[test]
fn test_authz_policy_svc_factory_exists() {
    assert_eq!(AUTHZ_POLICY_SVC_FACTORY, ());
}
