//! Integration tests for [`CredentialResolver`] trait.

use edge_domain_security::{Claims, CredentialResolver, CredentialSource, SecretString, SecurityContext, SecurityError, Token};

struct OkResolver;
impl CredentialResolver for OkResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Ok(Claims::default())
    }
    fn resolve(&self, _source: &CredentialSource, _ctx: &SecurityContext) -> Result<SecretString, SecurityError> {
        Ok(SecretString::new("secret"))
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
    let result = resolver.verify(&Token::new("valid"));
    assert!(result.is_ok());
}

/// @covers: verify
#[test]
fn test_credential_resolver_verify_error() {
    let resolver = FailResolver;
    let result = resolver.verify(&Token::new("invalid"));
    assert!(result.is_err());
}

/// @covers: verify
#[test]
fn test_credential_resolver_verify_edge() {
    let resolver = OkResolver;
    let r1 = resolver.verify(&Token::new(""));
    let r2 = resolver.verify(&Token::new("x"));
    assert!(r1.is_ok() && r2.is_ok());
}

/// @covers: resolve
#[test]
fn test_credential_resolver_resolve_happy() {
    let resolver = OkResolver;
    let ctx = SecurityContext::unauthenticated();
    let result = resolver.resolve(&CredentialSource::new("test"), &ctx);
    assert!(result.is_ok());
}

/// @covers: resolve
#[test]
fn test_credential_resolver_resolve_error() {
    let resolver = FailResolver;
    let ctx = SecurityContext::unauthenticated();
    let result = resolver.resolve(&CredentialSource::new("test"), &ctx);
    assert!(result.is_err());
}

/// @covers: resolve
#[test]
fn test_credential_resolver_resolve_edge() {
    let resolver = OkResolver;
    let ctx = SecurityContext::unauthenticated();
    let r1 = resolver.resolve(&CredentialSource::new(""), &ctx);
    let r2 = resolver.resolve(&CredentialSource::new("x"), &ctx);
    assert!(r1.is_ok() && r2.is_ok());
}
