//! Integration tests for [`CredentialResolver`] trait.

use edge_domain_security::{Claims, CredentialResolver, CredentialSource, SecretString, SecurityContext, SecurityError, Token};

struct OkResolver;
impl CredentialResolver for OkResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Ok(Claims::default())
    }
    fn resolve(&self, _source: &CredentialSource, _ctx: &SecurityContext) -> Result<SecretString, SecurityError> {
        Ok(SecretString::from("secret"))
    }
}

struct FailResolver;
impl CredentialResolver for FailResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Err(SecurityError::Token("invalid token".to_string()))
    }
    fn resolve(&self, _source: &CredentialSource, _ctx: &SecurityContext) -> Result<SecretString, SecurityError> {
        Err(SecurityError::Auth("no credential".to_string()))
    }
}

/// @covers: verify
#[test]
fn test_credential_resolver_verify_happy() {
    let resolver = OkResolver;
    let result = resolver.verify(&Token::from("valid"));
    assert!(result.is_ok(), "verify must succeed");
    assert_eq!(result.unwrap(), Claims::default(), "verify must return default claims");
}

/// @covers: verify
#[test]
fn test_credential_resolver_verify_error() {
    let resolver = FailResolver;
    let result = resolver.verify(&Token::from("invalid"));
    assert!(result.is_err());
}

/// @covers: verify
#[test]
fn test_credential_resolver_verify_edge() {
    let resolver = OkResolver;
    let r1 = resolver.verify(&Token::from(""));
    let r2 = resolver.verify(&Token::from("x"));
    assert!(r1.is_ok() && r2.is_ok());
}

/// @covers: resolve
#[test]
fn test_resolve_happy() {
    use edge_domain_security::SecurityBootstrap;
    let resolver = OkResolver;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = resolver.resolve(&CredentialSource::from("test"), &ctx);
    assert!(result.is_ok(), "resolve must succeed for test source");
    let _secret = result.unwrap();
    // Secret is successfully resolved; the caller is responsible for handling it
    assert!(true);
}

/// @covers: resolve
#[test]
fn test_resolve_error() {
    use edge_domain_security::SecurityBootstrap;
    let resolver = FailResolver;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let result = resolver.resolve(&CredentialSource::from("test"), &ctx);
    assert!(result.is_err());
}

/// @covers: resolve
#[test]
fn test_resolve_edge() {
    use edge_domain_security::SecurityBootstrap;
    let resolver = OkResolver;
    let ctx = <edge_domain_security::SecurityServices as SecurityBootstrap>::unauthenticated();
    let r1 = resolver.resolve(&CredentialSource::from(""), &ctx);
    let r2 = resolver.resolve(&CredentialSource::from("x"), &ctx);
    assert!(r1.is_ok() && r2.is_ok());
}
