//! [`TlsConfig`] implementation for [`IngressTlsConfig`].

use crate::api::{IngressTlsConfig, IngressTlsError, TlsConfig};

impl IngressTlsConfig {
    /// Validate that all required paths are non-empty.
    pub(crate) fn validate(&self) -> Result<(), IngressTlsError> {
        if self.cert_pem_path.is_empty() {
            return Err(IngressTlsError::Config("certificate path is empty".into()));
        }
        if self.key_pem_path.is_empty() {
            return Err(IngressTlsError::Config("key path is empty".into()));
        }
        if let Some(ca) = &self.client_ca_pem_path {
            if ca.is_empty() {
                return Err(IngressTlsError::Config("CA path is empty".into()));
            }
        }
        Ok(())
    }
}

impl TlsConfig for IngressTlsConfig {
    fn ingress_tls(&self) -> &IngressTlsConfig {
        self
    }

    fn validate_tls(&self) -> Result<(), IngressTlsError> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[test]
    fn test_validate_empty_cert_returns_error() {
        let cfg = IngressTlsConfig::tls("", "k.pem");
        assert!(cfg.validate().is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_empty_key_returns_error() {
        let cfg = IngressTlsConfig::tls("c.pem", "");
        assert!(cfg.validate().is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_valid_config_ok() {
        let result = IngressTlsConfig::tls("c.pem", "k.pem").validate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }
}
