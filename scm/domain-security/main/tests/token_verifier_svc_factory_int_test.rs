//! Integration tests for TokenVerifier service factory.

use edge_domain_security::TOKEN_VERIFIER_SVC_FACTORY;

#[test]
fn test_token_verifier_svc_factory_exists() {
    assert_eq!(TOKEN_VERIFIER_SVC_FACTORY, ());
}
