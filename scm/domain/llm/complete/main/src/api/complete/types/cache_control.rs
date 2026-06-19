use serde::{Deserialize, Serialize};

/// Anthropic prompt-caching hint attached to a message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheControl {
    /// Cache control type (serialised as `"type"` in JSON).
    #[serde(rename = "type")]
    pub cache_type: String,
}

impl CacheControl {
    /// Construct a cache control of the given type string.
    pub fn new(cache_type: impl Into<String>) -> Self {
        Self { cache_type: cache_type.into() }
    }

    /// The `"ephemeral"` cache control type used for Anthropic prompt caching.
    pub fn ephemeral() -> Self {
        Self::new("ephemeral")
    }
}
