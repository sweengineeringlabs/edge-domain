use crate::api::reasoning::types::ReasoningPattern;

/// Response for [`Reasoning::supported_patterns`](crate::api::reasoning::traits::Reasoning::supported_patterns).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedPatternsResponse {
    /// Patterns the reasoner can execute.
    pub patterns: Vec<ReasoningPattern>,
}
