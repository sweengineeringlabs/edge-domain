//! Noop [`CredentialSourceResolver`] implementation.

use crate::api::CredentialSource;
use crate::api::CredentialSourceConfig;
use crate::api::CredentialSourceResolver;
use crate::api::SecurityError;

/// Noop credential source resolver that returns env-based source.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NoopCredentialSourceResolver;

impl CredentialSourceResolver for NoopCredentialSourceResolver {
    fn resolve(&self, _config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError> {
        Ok(CredentialSource::from("noop-token"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_credential_source_resolver_returns_env_source() {
        let resolver = NoopCredentialSourceResolver;
        let config = CredentialSourceConfig::default();
        let result = resolver.resolve(&config);
        assert!(result.is_ok());
        let source = result.unwrap();
        assert_eq!(source, CredentialSource::from("noop-token"));
    }
}
