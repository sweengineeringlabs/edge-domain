use crate::api::reasoning::types::PatternMetadata;

/// Response for [`Reasoning::pattern_metadata`](crate::api::reasoning::traits::Reasoning::pattern_metadata).
#[derive(Debug, Clone)]
pub struct PatternMetadataLookupResponse {
    /// Metadata for the queried pattern, if supported.
    pub metadata: Option<Box<PatternMetadata>>,
}
