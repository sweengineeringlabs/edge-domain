//! [`JwtKey`] — key material for JWT signature verification.

use serde::{Deserialize, Serialize};

/// Key material and algorithm used to verify JWT tokens.
///
/// Choose the variant that matches the algorithm your identity provider uses.
/// Serializes with a `"algorithm"` discriminant in TOML/JSON, matching the
/// format used by `swe-edge-ingress-verifier` for drop-in config compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "algorithm", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JwtKey {
    /// HMAC-SHA256 — shared symmetric secret bytes.
    Hs256 {
        /// Raw HMAC secret bytes.
        secret: Vec<u8>,
    },
    /// RSA-SHA256 — PEM-encoded RSA public key.
    Rs256 {
        /// PEM-encoded RSA public key (UTF-8 string).
        public_pem: String,
    },
    /// ECDSA-SHA256 — PEM-encoded EC public key.
    Es256 {
        /// PEM-encoded EC public key (UTF-8 string).
        public_pem: String,
    },
}
