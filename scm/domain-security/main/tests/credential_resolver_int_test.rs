//! Integration tests for CredentialResolver trait.

use edge_domain_security::{
    Claims, CredentialResolver, CredentialSource, SecretString, SecurityContext, Token,
    SecurityError,
};

struct SuccessResolver;
impl CredentialResolver for SuccessResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Ok(Claims::default())
    }

    fn resolve(
        &self,
        _source: &CredentialSource,
        _ctx: &SecurityContext,
    ) -> Result<SecretString, SecurityError> {
        Ok(SecretString::from("test-secret"))
    }
}

struct FailResolver;
impl CredentialResolver for FailResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Err(SecurityError::Auth("invalid token".to_string()))
    }

    fn resolve(
        &self,
        _source: &CredentialSource,
        _ctx: &SecurityContext,
    ) -> Result<SecretString, SecurityError> {
        Err(SecurityError::Auth("no credential".to_string()))
    }
}

/// @covers: CredentialResolver::verify
#[test]
fn test_verify_valid_happy() {
    let resolver = SuccessResolver;
    let token = Token::from("test-token");
    let result = resolver.verify(&token);
    let claims = result.unwrap();
    assert_eq!(claims, Claims::default());
}

/// @covers: CredentialResolver::verify
#[test]
fn test_verify_invalid_error() {
    let resolver = FailResolver;
    let token = Token::from("bad-token");
    assert!(resolver.verify(&token).is_err());
}

/// @covers: CredentialResolver::verify
#[test]
fn test_verify_empty_edge() {
    let resolver = SuccessResolver;
    let token = Token::from("");
    let result = resolver.verify(&token);
    assert!(result.is_ok());
}

/// @covers: CredentialResolver::resolve
#[test]
fn test_resolve_credential_happy() {
    let resolver = SuccessResolver;
    let source = CredentialSource::from("test");
    let ctx = SecurityContext::unauthenticated();
    let result = resolver.resolve(&source, &ctx);
    let secret = result.unwrap();
    assert_eq!(secret.expose(), "test-secret");
}

/// @covers: CredentialResolver::resolve
#[test]
fn test_resolve_missing_error() {
    let resolver = FailResolver;
    let source = CredentialSource::from("test");
    let ctx = SecurityContext::unauthenticated();
    assert!(resolver.resolve(&source, &ctx).is_err());
}

/// @covers: CredentialResolver::resolve
#[test]
fn test_resolve_authenticated_edge() {
    let resolver = SuccessResolver;
    let source = CredentialSource::from("service");
    let ctx = SecurityContext::unauthenticated();
    let result = resolver.resolve(&source, &ctx);
    let secret = result.unwrap();
    assert_eq!(secret.expose(), "test-secret");
}
