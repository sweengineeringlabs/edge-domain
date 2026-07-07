use serde::{Deserialize, Serialize};

/// Cache entry for a rendered prompt.
///
/// Orphan-type note: `Prompt::cache` takes/returns `CacheBuildRequest`/`CacheBuildResponse`
/// (flattened fields), never `PromptCache` directly, so `no_orphan_types` flags this as
/// unreferenced. It is a plain data struct with no interface behind it — inventing a trait
/// solely to reference it would be ceremony with no real polymorphism.
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
