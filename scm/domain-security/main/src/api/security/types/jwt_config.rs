//! [`JwtConfig`] — configuration for JWT token verification.

use serde::{Deserialize, Serialize};

use super::jwt_key::JwtKey;

/// Configuration for JWT token verification.
///
/// Deserializes directly from TOML. `required_issuer` and `required_audience`
/// are optional — omit them to skip those claim checks. `leeway_seconds`
/// allows a clock skew tolerance (default 0).
///
/// Field names and serde representation are compatible with the equivalent
/// type in `swe-edge-ingress-verifier`, so existing TOML files require no changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Key material and algorithm for signature verification.
    pub key: JwtKey,
    /// Required `iss` claim value. `None` skips issuer validation.
    #[serde(default)]
    pub required_issuer: Option<String>,
    /// Required `aud` claim value. `None` skips audience validation.
    #[serde(default)]
    pub required_audience: Option<String>,
    /// Maximum clock skew in seconds for `exp`/`nbf` checks.
    #[serde(default)]
    pub leeway_seconds: u64,
}
