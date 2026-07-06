//! `PatternMetadataBuilder` — fluent builder for [`PatternMetadata`].

use crate::api::reasoning::types::ReasoningPattern;

/// Fluent builder for [`PatternMetadata`].
#[derive(Clone, Debug)]
pub struct PatternMetadataBuilder {
    pub(crate) pattern: ReasoningPattern,
    pub(crate) max_depth: usize,
    pub(crate) max_tokens: usize,
    pub(crate) min_confidence: f32,
    pub(crate) allow_backtracking: bool,
    pub(crate) timeout_secs: u64,
    pub(crate) tags: Vec<String>,
}
