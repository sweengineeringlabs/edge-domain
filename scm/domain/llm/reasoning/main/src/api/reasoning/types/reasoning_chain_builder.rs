//! `ReasoningChainBuilder` — fluent builder for [`ReasoningChain`].

use crate::api::reasoning::types::ThinkingProcess;

/// Fluent builder for [`ReasoningChain`].
///
/// Orphan-type note: exposes its behavior via inherent builder methods, not by implementing
/// a trait, so `no_orphan_types` flags it as unreferenced — accepted tradeoff, same rationale
/// as `StdReasoningFactory`.
#[derive(Clone, Debug)]
pub struct ReasoningChainBuilder {
    pub(crate) id: String,
    pub(crate) processes: Vec<ThinkingProcess>,
    pub(crate) is_complete: bool,
    pub(crate) final_answer: Option<String>,
}
