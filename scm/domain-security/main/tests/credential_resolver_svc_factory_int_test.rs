//! Integration tests for CredentialResolver service factory.

use edge_domain_security::CREDENTIAL_RESOLVER_SVC_FACTORY;

#[test]
fn test_credential_resolver_svc_factory_exists() {
    assert_eq!(CREDENTIAL_RESOLVER_SVC_FACTORY, ());
}
