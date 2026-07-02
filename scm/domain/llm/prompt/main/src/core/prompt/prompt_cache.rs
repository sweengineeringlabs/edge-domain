//! Inherent methods for [`PromptCache`].

use std::time::{Duration, SystemTime};

use crate::api::PromptCache;

impl PromptCache {
    /// Default TTL applied by [`PromptCache::new`] when none is overridden.
    pub const DEFAULT_TTL_SECONDS: u64 = 3600;

    /// Create a new cache entry
    pub fn new(key: String, rendered: String, token_count: usize) -> Self {
        Self {
            key,
            rendered,
            token_count,
            created_at: Self::now_secs(),
            ttl_seconds: Self::DEFAULT_TTL_SECONDS,
            hit_count: 0,
        }
    }

    /// Check if cache entry has expired
    pub fn is_expired(&self) -> bool {
        self.elapsed_secs() >= self.ttl_seconds
    }

    /// Get age of cache entry
    pub fn age(&self) -> Duration {
        Duration::from_secs(self.elapsed_secs())
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

    /// Current Unix time in seconds.
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Seconds elapsed since this entry was created.
    fn elapsed_secs(&self) -> u64 {
        Self::now_secs().saturating_sub(self.created_at)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_default_ttl_one_hour() {
        let c = PromptCache::new("k".into(), "r".into(), 3);
        assert_eq!(c.ttl_seconds, 3600);
    }

    /// @covers: is_expired
    #[test]
    fn test_is_expired_zero_ttl_is_expired() {
        let c = PromptCache::new("k".into(), "r".into(), 3).with_ttl(0);
        assert!(c.is_expired());
    }

    /// @covers: age
    #[test]
    fn test_age_fresh_entry_is_near_zero() {
        let c = PromptCache::new("k".into(), "r".into(), 3);
        assert!(c.age().as_secs() < 5);
    }

    /// @covers: record_hit
    #[test]
    fn test_record_hit_increments_count() {
        let mut c = PromptCache::new("k".into(), "r".into(), 3);
        c.record_hit();
        assert_eq!(c.hit_count, 1);
    }

    /// @covers: with_ttl
    #[test]
    fn test_with_ttl_overrides_default() {
        let c = PromptCache::new("k".into(), "r".into(), 3).with_ttl(60);
        assert_eq!(c.ttl_seconds, 60);
    }

    /// @covers: elapsed_secs
    #[test]
    fn test_elapsed_secs_fresh_entry_is_near_zero() {
        let c = PromptCache::new("k".into(), "r".into(), 3);
        assert!(c.elapsed_secs() < 5);
    }
}
