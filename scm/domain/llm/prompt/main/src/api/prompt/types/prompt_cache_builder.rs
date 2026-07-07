//! `PromptCacheBuilder` — fluent builder for [`PromptCache`].

/// Fluent builder for [`PromptCache`](crate::api::prompt::types::PromptCache).
///
/// Orphan-type note: same rationale as [`PromptCache`](crate::api::prompt::types::PromptCache)
/// itself — a plain builder, never a trait method parameter or return type.
#[derive(Clone, Debug, Default)]
pub struct PromptCacheBuilder {
    pub(crate) key: String,
    pub(crate) rendered: String,
    pub(crate) token_count: usize,
    pub(crate) ttl_seconds: Option<u64>,
    pub(crate) hit_count: u32,
}
