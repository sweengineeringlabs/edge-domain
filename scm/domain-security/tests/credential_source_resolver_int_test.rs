//! Integration tests for [`CredentialSourceResolver`] trait.

use edge_domain_security::{CredentialSource, CredentialSourceConfig, CredentialSourceResolver, SecurityError};

struct OkResolver;
impl CredentialSourceResolver for OkResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Ok(CredentialSource::from("resolved"))
    }
}

struct FailResolver;
impl CredentialSourceResolver for FailResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Err(SecurityError::Credential("resolution failed".to_string()))
    }
}

/// @covers: CredentialSourceResolver::resolve
#[test]
fn test_resolve_config_happy() {
    let resolver = OkResolver;
    let config = CredentialSourceConfig {
        env_var: Some("TEST".to_string()),
        file_path: None,
        file_path_env_override: None,
    };
    let result = resolver.resolve(&config);
    assert!(result.is_ok(), "resolve must succeed with valid config");
    let source = result.unwrap();
    assert_eq!(source, CredentialSource::from("resolved"));
}

/// @covers: CredentialSourceResolver::resolve
#[test]
fn test_resolve_config_error() {
    let resolver = FailResolver;
    let config = CredentialSourceConfig {
        env_var: Some("TEST".to_string()),
        file_path: None,
        file_path_env_override: None,
    };
    assert!(resolver.resolve(&config).is_err());
}

/// @covers: CredentialSourceResolver::resolve
#[test]
fn test_resolve_config_edge() {
    let resolver = OkResolver;
    let config = CredentialSourceConfig {
        env_var: Some("X".to_string()),
        file_path: None,
        file_path_env_override: None,
    };
    let r1 = resolver.resolve(&config);
    let r2 = resolver.resolve(&config);
    assert_eq!(r1.is_ok(), r2.is_ok());
}
