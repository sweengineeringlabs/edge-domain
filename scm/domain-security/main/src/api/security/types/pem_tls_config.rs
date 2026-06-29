//! [`PemTlsConfig`] — TLS/mTLS server configuration for inbound connections.

use serde::{Deserialize, Serialize};

/// TLS or mTLS configuration for a server endpoint.
///
/// Specifies PEM file paths for the server certificate chain and private key,
/// plus an optional CA for mutual TLS.
/// Set `ca_pem_path` to `Some(path)` to enable mutual TLS.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PemTlsConfig {
    /// Path to the PEM file containing the server certificate chain (leaf first).
    pub cert_pem_path: String,
    /// Path to the PEM file containing the server's private key.
    pub key_pem_path: String,
    /// CA certificate path; `Some` enables mutual TLS.
    pub ca_pem_path: Option<String>,
}
