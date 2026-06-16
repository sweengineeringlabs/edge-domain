use serde::{Deserialize, Serialize};

/// Anthropic prompt-caching hint attached to a message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: String,
}

impl CacheControl {
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".into(),
        }
    }

    pub fn custom(cache_type: impl Into<String>) -> Self {
        Self {
            cache_type: cache_type.into(),
        }
    }

    pub fn is_ephemeral(&self) -> bool {
        self.cache_type == "ephemeral"
    }
}
