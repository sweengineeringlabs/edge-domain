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

#[test]
fn test_credential_resolver_verify_happy() {
    let resolver = SuccessResolver;
    let token = Token::bearer("test-token".to_string());
    let result = resolver.verify(&token);
    let claims = result.unwrap();
    assert_eq!(claims, Claims::default());
}

#[test]
fn test_credential_resolver_verify_error() {
    let resolver = FailResolver;
    let token = Token::bearer("bad-token".to_string());
    assert!(resolver.verify(&token).is_err());
}

#[test]
fn test_credential_resolver_resolve_happy() {
    let resolver = SuccessResolver;
    let source = CredentialSource::from("test");
    let ctx = SecurityContext::unauthenticated();
    let result = resolver.resolve(&source, &ctx);
    let secret = result.unwrap();
    assert_eq!(secret.expose(), "test-secret");
}

#[test]
fn test_credential_resolver_resolve_error() {
    let resolver = FailResolver;
    let source = CredentialSource::from("test");
    let ctx = SecurityContext::unauthenticated();
    assert!(resolver.resolve(&source, &ctx).is_err());
}
