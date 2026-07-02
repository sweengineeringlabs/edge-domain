use crate::api::reasoning::types::ReasoningPattern;

/// Request for [`Reasoning::reason`](crate::api::reasoning::traits::Reasoning::reason).
#[derive(Debug, Clone, Copy)]
pub struct ReasonRequest<'a> {
    /// Problem statement to reason about.
    pub problem: &'a str,
    /// Reasoning pattern to apply.
    pub pattern: ReasoningPattern,
}
