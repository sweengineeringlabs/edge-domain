//! Noop [`CredentialResolver`] implementation.

use crate::api::Claims;
use crate::api::CredentialResolver;
use crate::api::CredentialSource;
use crate::api::SecretString;
use crate::api::SecurityContext;
use crate::api::SecurityError;
use crate::api::Token;

/// Noop credential resolver that rejects all operations.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NoopCredentialResolver;

impl CredentialResolver for NoopCredentialResolver {
    fn verify(&self, _token: &Token) -> Result<Claims, SecurityError> {
        Err(SecurityError::Auth(
            "noop resolver rejects all tokens".to_string(),
        ))
    }

    fn resolve(
        &self,
        _source: &CredentialSource,
        _ctx: &SecurityContext,
    ) -> Result<SecretString, SecurityError> {
        Err(SecurityError::Auth(
            "noop resolver has no credentials".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_credential_resolver_rejects_verify() {
        let resolver = NoopCredentialResolver;
        let token = Token::from("test-token");
        assert!(resolver.verify(&token).is_err());
    }

    #[test]
    fn test_noop_credential_resolver_rejects_resolve() {
        let resolver = NoopCredentialResolver;
        let source = CredentialSource::from("test");
        let ctx = SecurityContext::unauthenticated();
        assert!(resolver.resolve(&source, &ctx).is_err());
    }
}
