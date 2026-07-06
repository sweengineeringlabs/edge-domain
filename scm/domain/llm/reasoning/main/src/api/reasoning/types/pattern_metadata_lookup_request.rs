use crate::api::reasoning::types::ReasoningPattern;

/// Request for [`Reasoning::pattern_metadata`](crate::api::reasoning::traits::Reasoning::pattern_metadata).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternMetadataLookupRequest {
    /// Pattern to look up metadata for.
    pub pattern: ReasoningPattern,
}
