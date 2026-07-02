//! `ReasoningChainBuilder` — fluent builder for [`ReasoningChain`].

use crate::api::reasoning::types::ThinkingProcess;

/// Fluent builder for [`ReasoningChain`].
#[derive(Clone, Debug)]
pub struct ReasoningChainBuilder {
    pub(crate) id: String,
    pub(crate) processes: Vec<ThinkingProcess>,
    pub(crate) is_complete: bool,
    pub(crate) final_answer: Option<String>,
}
