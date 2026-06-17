use serde::{Deserialize, Serialize};

/// Anthropic prompt-caching hint attached to a message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheControl {
    /// The caching strategy type (e.g. "ephemeral").
    #[serde(rename = "type")]
    pub cache_type: String,
}

impl CacheControl {
    /// Creates an ephemeral cache control hint.
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".into(),
        }
    }

    /// Creates a cache control hint with a custom cache type.
    pub fn custom(cache_type: impl Into<String>) -> Self {
        Self {
            cache_type: cache_type.into(),
        }
    }

    /// Returns true if this cache control is ephemeral.
    pub fn is_ephemeral(&self) -> bool {
        self.cache_type == "ephemeral"
    }
}
