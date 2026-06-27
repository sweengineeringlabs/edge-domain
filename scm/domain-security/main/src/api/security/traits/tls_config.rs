//! [`TlsConfig`] — trait contract for TLS/mTLS server configuration.

use crate::api::security::errors::ingress_tls_error::IngressTlsError;
use crate::api::security::types::ingress_tls_config::IngressTlsConfig;

/// Contract for types that carry and can validate TLS inbound-connection configuration.
pub trait TlsConfig {
    /// Return the TLS configuration for this endpoint.
    fn ingress_tls(&self) -> &IngressTlsConfig;

    /// Validate the TLS configuration, returning an error on misconfiguration.
    fn validate_tls(&self) -> Result<(), IngressTlsError>;

    /// Return `true` if this is a mutual-TLS configuration (client CA path is set).
    fn is_mtls(&self) -> bool {
        self.ingress_tls().client_ca_pem_path.is_some()
    }

    /// The server certificate PEM file path.
    fn cert_path(&self) -> &str {
        &self.ingress_tls().cert_pem_path
    }

    /// The server private key PEM file path.
    fn key_path(&self) -> &str {
        &self.ingress_tls().key_pem_path
    }

    /// The client CA certificate PEM path, or `None` for plain TLS.
    fn ca_path(&self) -> Option<&str> {
        self.ingress_tls().client_ca_pem_path.as_deref()
    }
}
