//! Integration tests for CredentialSourceResolver svc factory.

/// @covers: CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY
#[test]
fn test_credential_source_resolver_svc_factory_exists() {
    let _ = edge_domain_security::CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
}
