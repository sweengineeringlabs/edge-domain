//! Integration tests for CredentialResolver svc factory.

/// @covers: CREDENTIAL_RESOLVER_SVC_FACTORY
#[test]
fn test_credential_resolver_svc_factory_exists() {
    let _ = edge_domain_security::CREDENTIAL_RESOLVER_SVC_FACTORY;
}
