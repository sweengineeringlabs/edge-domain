//! Integration tests for AuthzPolicy svc factory.

/// @covers: AUTHZ_POLICY_SVC_FACTORY
#[test]
fn test_authz_policy_svc_factory_exists() {
    let _ = edge_domain_security::AUTHZ_POLICY_SVC_FACTORY;
}
