//! Integration tests for CredentialSourceResolver trait.

use edge_domain_security::{
    CredentialSource, CredentialSourceConfig, CredentialSourceResolver, SecurityError,
};

struct SuccessSourceResolver;
impl CredentialSourceResolver for SuccessSourceResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Ok(CredentialSource::from("test-source"))
    }
}

struct FailSourceResolver;
impl CredentialSourceResolver for FailSourceResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Err(SecurityError::Credential("no source".to_string()))
    }
}

#[test]
fn test_credential_source_resolver_resolve_happy() {
    let resolver = SuccessSourceResolver;
    let config = CredentialSourceConfig::default();
    let result = resolver.resolve(&config);
    let source = result.unwrap();
    assert_eq!(source, CredentialSource::from("test-source"));
}

#[test]
fn test_credential_source_resolver_resolve_error() {
    let resolver = FailSourceResolver;
    let config = CredentialSourceConfig::default();
    assert!(resolver.resolve(&config).is_err());
}

#[test]
fn test_credential_source_resolver_resolve_edge() {
    let resolver = SuccessSourceResolver;
    let config = CredentialSourceConfig::default();
    let source = resolver.resolve(&config).unwrap();
    assert_eq!(source, CredentialSource::from("test-source"));
}
