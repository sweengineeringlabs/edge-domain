//! `CacheableMessage` — Anthropic prompt-caching hint attachment contract.

use crate::api::complete::types::CacheControl;

/// Types that can carry Anthropic prompt-caching hints.
pub trait CacheableMessage: Sized {
    /// Attach the given cache control hint to this message.
    fn with_cache_control(self, cache: CacheControl) -> Self;

    /// Convenience: mark this message as ephemeral (short-lived cache entry).
    fn mark_ephemeral(self) -> Self {
        self.with_cache_control(CacheControl::ephemeral())
    }
}
