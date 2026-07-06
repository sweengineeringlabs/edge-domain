use serde::{Deserialize, Serialize};

/// Anthropic prompt-caching hint attached to a message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheControl {
    /// Cache control type (serialised as `"type"` in JSON).
    #[serde(rename = "type")]
    pub cache_type: String,
}
