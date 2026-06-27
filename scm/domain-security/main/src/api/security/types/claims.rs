//! [`Claims`] — JWT standard claims payload.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Standard JWT claims extracted from a verified token.
///
/// All fields are optional — JWT does not mandate any particular claim.
/// Non-standard claims (e.g. `tenant_id`, `roles`) are captured in `custom`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    /// Subject — the entity the token represents (e.g. user ID, service name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Issuer — who issued the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Audience — who the token is intended for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<Vec<String>>,
    /// Expiration time (Unix timestamp seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    /// Issued-at time (Unix timestamp seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    /// Not-before time (Unix timestamp seconds) — token not valid before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    /// JWT ID — unique identifier for this token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    /// Non-standard claims (e.g. `tenant_id`, `roles`, custom application claims).
    #[serde(flatten)]
    pub custom: HashMap<String, Value>,
}
