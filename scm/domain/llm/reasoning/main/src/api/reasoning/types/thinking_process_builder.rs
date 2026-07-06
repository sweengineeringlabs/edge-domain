//! `ThinkingProcessBuilder` — fluent builder for [`ThinkingProcess`].

use crate::api::reasoning::types::ReasoningStep;

/// Fluent builder for [`ThinkingProcess`](crate::api::reasoning::types::ThinkingProcess).
#[derive(Clone, Debug)]
pub struct ThinkingProcessBuilder {
    pub(crate) id: String,
    pub(crate) problem: String,
    pub(crate) steps: Vec<ReasoningStep>,
    pub(crate) is_complete: bool,
    pub(crate) conclusion: Option<String>,
}
