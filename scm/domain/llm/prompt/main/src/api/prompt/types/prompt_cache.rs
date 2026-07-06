use serde::{Deserialize, Serialize};

/// Cache entry for a rendered prompt
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptCache {
    /// Unique cache key (hash of template + context)
    pub key: String,

    /// Cached rendered prompt
    pub rendered: String,

    /// Token count of rendered prompt
    pub token_count: usize,

    /// When this cache entry was created (Unix seconds)
    pub created_at: u64,

    /// TTL for this cache entry (seconds)
    pub ttl_seconds: u64,

    /// Number of times this cache entry has been used
    pub hit_count: u32,
}
