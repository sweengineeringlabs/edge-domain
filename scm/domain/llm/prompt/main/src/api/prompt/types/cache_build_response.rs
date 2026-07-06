//! [`CacheBuildResponse`] — response for [`Prompt::cache`](crate::api::prompt::traits::Prompt::cache).

/// The built cache entry, flattened from `PromptCache` (SEA `field_type_purity`:
/// a freshly-computed value with no borrow source cannot be nested by reference).
// @allow: suggest_builder_pattern — a Response DTO, always fully constructed
// in one shot by `Prompt::cache`; never partially built by callers.
#[derive(Debug, PartialEq)]
pub struct CacheBuildResponse {
    /// Unique cache key (hash of template + context).
    pub key: String,
    /// Cached rendered prompt.
    pub rendered: String,
    /// Token count of the rendered prompt.
    pub token_count: usize,
    /// When this cache entry was created (Unix seconds).
    pub created_at: u64,
    /// TTL for this cache entry (seconds).
    pub ttl_seconds: u64,
    /// Number of times this cache entry has been used.
    pub hit_count: u32,
}
