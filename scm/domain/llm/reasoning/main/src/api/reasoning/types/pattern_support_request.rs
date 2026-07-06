use crate::api::reasoning::types::ReasoningPattern;

/// Request for [`Reasoning::supports_pattern`](crate::api::reasoning::traits::Reasoning::supports_pattern).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PatternSupportRequest {
    /// Pattern being queried.
    pub pattern: ReasoningPattern,
}
