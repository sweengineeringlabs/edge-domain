//! Integration tests for TokenVerifier svc factory.

/// @covers: TOKEN_VERIFIER_SVC_FACTORY
#[test]
fn test_token_verifier_svc_factory_exists() {
    let _ = edge_domain_security::TOKEN_VERIFIER_SVC_FACTORY;
}
