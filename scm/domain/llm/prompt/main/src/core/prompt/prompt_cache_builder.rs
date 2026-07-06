//! Inherent methods for [`PromptCacheBuilder`].

use crate::api::{PromptCache, PromptCacheBuilder};

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
        let ttl_seconds = self.effective_ttl();
        let mut cache = PromptCache::new(self.key, self.rendered, self.token_count);
        cache.ttl_seconds = ttl_seconds;
        cache.hit_count = self.hit_count;
        cache
    }

    /// The TTL to apply: the explicit override, or `PromptCache`'s own default.
    fn effective_ttl(&self) -> u64 {
        self.ttl_seconds.unwrap_or(PromptCache::DEFAULT_TTL_SECONDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_key() {
        assert_eq!(PromptCacheBuilder::new().key, "");
    }

    /// @covers: build
    #[test]
    fn test_build_applies_all_overrides() {
        let c = PromptCacheBuilder::new()
            .key("k".into())
            .rendered("r".into())
            .token_count(3)
            .ttl_seconds(60)
            .hit_count(2)
            .build();
        assert_eq!(c.key, "k");
        assert_eq!(c.ttl_seconds, 60);
        assert_eq!(c.hit_count, 2);
    }

    /// @covers: effective_ttl
    #[test]
    fn test_effective_ttl_falls_back_to_cache_default_when_unset() {
        let b = PromptCacheBuilder::new();
        assert_eq!(b.effective_ttl(), 3600);
    }

    /// @covers: effective_ttl
    #[test]
    fn test_effective_ttl_uses_override_when_set() {
        let b = PromptCacheBuilder::new().ttl_seconds(120);
        assert_eq!(b.effective_ttl(), 120);
    }

    /// @covers: key
    #[test]
    fn test_key_sets_field() {
        assert_eq!(PromptCacheBuilder::new().key("k".into()).build().key, "k");
    }

    /// @covers: rendered
    #[test]
    fn test_rendered_sets_field() {
        assert_eq!(
            PromptCacheBuilder::new()
                .rendered("r".into())
                .build()
                .rendered,
            "r"
        );
    }

    /// @covers: token_count
    #[test]
    fn test_token_count_sets_field() {
        assert_eq!(
            PromptCacheBuilder::new().token_count(9).build().token_count,
            9
        );
    }

    /// @covers: ttl_seconds
    #[test]
    fn test_ttl_seconds_overrides_default() {
        assert_eq!(
            PromptCacheBuilder::new()
                .ttl_seconds(42)
                .build()
                .ttl_seconds,
            42
        );
    }

    /// @covers: hit_count
    #[test]
    fn test_hit_count_sets_field() {
        assert_eq!(PromptCacheBuilder::new().hit_count(5).build().hit_count, 5);
    }
}
