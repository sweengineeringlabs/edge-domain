//! [`IngressTlsConfig`] — TLS/mTLS server configuration for inbound connections.

use serde::{Deserialize, Serialize};

/// TLS or mTLS configuration for a server endpoint.
///
/// Specifies PEM file paths for the server certificate chain and private key,
/// plus an optional client CA for mutual TLS.
/// Set `client_ca_pem_path` to `Some(path)` to enable mutual TLS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngressTlsConfig {
    /// Path to the PEM file containing the server certificate chain (leaf first).
    pub cert_pem_path: String,
    /// Path to the PEM file containing the server's private key.
    pub key_pem_path: String,
    /// Client CA certificate path; `Some` enables mutual TLS.
    pub client_ca_pem_path: Option<String>,
}

impl IngressTlsConfig {
    /// TLS-only: server authenticates with `cert`/`key`; client certificates are not required.
    pub fn tls(cert_pem_path: impl Into<String>, key_pem_path: impl Into<String>) -> Self {
        Self {
            cert_pem_path: cert_pem_path.into(),
            key_pem_path: key_pem_path.into(),
            client_ca_pem_path: None,
        }
    }

    /// mTLS: server authenticates with `cert`/`key`; clients must present a certificate
    /// signed by `client_ca`.
    pub fn mtls(
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
}
