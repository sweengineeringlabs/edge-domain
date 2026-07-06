use serde::{Deserialize, Serialize};

/// Anthropic prompt-caching hint attached to a message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheControl {
    /// The caching strategy type (e.g. "ephemeral").
    #[serde(rename = "type")]
    pub cache_type: String,
}
