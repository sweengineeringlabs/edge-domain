use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Cache entry for a rendered prompt
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl PromptCache {
    /// Create a new cache entry
    pub fn new(key: String, rendered: String, token_count: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            key,
            rendered,
            token_count,
            created_at: now,
            ttl_seconds: 3600, // Default 1 hour
            hit_count: 0,
        }
    }

    /// Check if cache entry has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        now.saturating_sub(self.created_at) >= self.ttl_seconds
    }

    /// Get age of cache entry
    pub fn age(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Duration::from_secs(now.saturating_sub(self.created_at))
    }

    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hit_count = self.hit_count.saturating_add(1);
    }

    /// Set custom TTL
    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        self.ttl_seconds = ttl_seconds;
        self
    }
}
