//! `PromptCacheBuilder` — fluent builder for [`PromptCache`].

/// Fluent builder for [`PromptCache`](crate::api::prompt::types::PromptCache).
#[derive(Clone, Debug, Default)]
pub struct PromptCacheBuilder {
    pub(crate) key: String,
    pub(crate) rendered: String,
    pub(crate) token_count: usize,
    pub(crate) ttl_seconds: Option<u64>,
    pub(crate) hit_count: u32,
}
