//! Integration tests for CredentialSourceResolver service factory.

use edge_domain_security::CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;

#[test]
fn test_credential_source_resolver_svc_factory_exists() {
    assert_eq!(CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY, ());
}
