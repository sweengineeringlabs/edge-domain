//! `PromptCacheBuilder` — fluent builder for [`PromptCache`].

use crate::api::prompt::types::PromptCache;

/// Fluent builder for [`PromptCache`].
#[derive(Clone, Debug, Default)]
pub struct PromptCacheBuilder {
    key: String,
    rendered: String,
    token_count: usize,
    ttl_seconds: Option<u64>,
    hit_count: u32,
}

impl PromptCacheBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the cache key.
    pub fn key(mut self, value: String) -> Self {
        self.key = value;
        self
    }

    /// Set the rendered prompt text.
    pub fn rendered(mut self, value: String) -> Self {
        self.rendered = value;
        self
    }

    /// Set the token count of the rendered prompt.
    pub fn token_count(mut self, value: usize) -> Self {
        self.token_count = value;
        self
    }

    /// Set a custom TTL in seconds.
    pub fn ttl_seconds(mut self, value: u64) -> Self {
        self.ttl_seconds = Some(value);
        self
    }

    /// Set the initial hit count.
    pub fn hit_count(mut self, value: u32) -> Self {
        self.hit_count = value;
        self
    }

    /// Build the [`PromptCache`].
    pub fn build(self) -> PromptCache {
        let mut cache = PromptCache::new(self.key, self.rendered, self.token_count);
        if let Some(ttl) = self.ttl_seconds {
            cache.ttl_seconds = ttl;
        }
        cache.hit_count = self.hit_count;
        cache
    }
}
