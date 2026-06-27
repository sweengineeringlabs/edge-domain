//! [`IngressTlsConfig`] constructors and [`TlsConfig`] implementation.

use crate::api::{IngressTlsConfig, IngressTlsError, TlsConfig};

impl IngressTlsConfig {
    /// TLS-only: server authenticates with `cert`/`key`; client certificates are not required.
    pub(crate) fn tls(cert_pem_path: impl Into<String>, key_pem_path: impl Into<String>) -> Self {
        Self {
            cert_pem_path: cert_pem_path.into(),
            key_pem_path: key_pem_path.into(),
            client_ca_pem_path: None,
        }
    }

    /// mTLS: server authenticates with `cert`/`key`; clients must present a certificate
    /// signed by `client_ca`.
    pub(crate) fn mtls(
        cert_pem_path: impl Into<String>,
        key_pem_path: impl Into<String>,
        client_ca_pem_path: impl Into<String>,
    ) -> Self {
        Self {
            cert_pem_path: cert_pem_path.into(),
            key_pem_path: key_pem_path.into(),
            client_ca_pem_path: Some(client_ca_pem_path.into()),
        }
    }

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

    /// @covers: tls
    #[test]
    fn test_tls_sets_cert_and_key() {
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        assert_eq!(cfg.cert_pem_path, "cert.pem");
        assert_eq!(cfg.key_pem_path, "key.pem");
        assert!(cfg.client_ca_pem_path.is_none());
    }

    /// @covers: mtls
    #[test]
    fn test_mtls_sets_cert_key_and_ca() {
        let cfg = IngressTlsConfig::mtls("c.pem", "k.pem", "ca.pem");
        assert_eq!(cfg.cert_pem_path, "c.pem");
        assert_eq!(cfg.client_ca_pem_path, Some("ca.pem".to_string()));
    }

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
