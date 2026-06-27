//! Integration tests for [`CredentialSourceResolver`] trait.

use edge_domain_security::{CredentialSource, CredentialSourceConfig, CredentialSourceResolver, SecurityError};

struct OkResolver;
impl CredentialSourceResolver for OkResolver {
    fn resolve(&self, config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        if config.is_empty() {
            Err(SecurityError::Credential("no config".to_string()))
        } else {
            Ok(CredentialSource::from("resolved"))
        }
    }
}

struct FailResolver;
impl CredentialSourceResolver for FailResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Err(SecurityError::Credential("resolution failed".to_string()))
    }
}

/// @covers: resolve
#[test]
fn test_credential_source_resolver_resolve_happy() {
    let resolver = OkResolver;
    let config = CredentialSourceConfig::new().with_env_var("TEST");
    assert!(resolver.resolve(&config).is_ok());
}

/// @covers: resolve
#[test]
fn test_credential_source_resolver_resolve_error() {
    let resolver = FailResolver;
    let config = CredentialSourceConfig::new().with_env_var("TEST");
    assert!(resolver.resolve(&config).is_err());
}

/// @covers: resolve
#[test]
fn test_credential_source_resolver_resolve_edge() {
    let resolver = OkResolver;
    let config = CredentialSourceConfig::new().with_env_var("X");
    let r1 = resolver.resolve(&config);
    let r2 = resolver.resolve(&config);
    assert_eq!(r1.is_ok(), r2.is_ok());
}
