//! `PatternMetadataBuilder` — fluent builder for [`PatternMetadata`].

use crate::api::reasoning::types::ReasoningPattern;

/// Fluent builder for [`PatternMetadata`].
///
/// Orphan-type note: exposes its behavior via inherent builder methods, not by implementing
/// a trait, so `no_orphan_types` flags it as unreferenced — accepted tradeoff, same rationale
/// as `StdReasoningFactory`.
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
