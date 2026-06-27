//! [`SecurityError`] — unified error type for security operations.

use thiserror::Error;

/// Top-level error for security operations across ingress and egress.
///
/// Each variant corresponds to one security concern. Crate-local error types
/// (`AuthError`, `IngressTlsError`, `TenantError`) implement `Into<SecurityError>`
/// so callers that compose multiple security stages can work with a single type.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SecurityError {
    /// Authentication or authorization failure.
    #[error("auth: {0}")]
    Auth(String),
    /// TLS configuration or handshake failure.
    #[error("tls: {0}")]
    Tls(String),
    /// Tenant identity resolution failure.
    #[error("tenant: {0}")]
    Tenant(String),
    /// Token validation or refresh failure.
    #[error("token: {0}")]
    Token(String),
    /// Token verification failure (inbound token is invalid/expired/tampered).
    #[error("verification: {0}")]
    Verification(String),
    /// Credential resolution failure (env var not found, file not accessible, etc.).
    #[error("credential: {0}")]
    Credential(String),
    /// Authorization policy failure (caller lacks required permissions).
    #[error("authz: {0}")]
    Authz(String),
}
