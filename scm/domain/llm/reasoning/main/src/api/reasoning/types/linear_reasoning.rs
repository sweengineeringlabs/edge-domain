//! `LinearReasoning` — reference [`Reasoning`](crate::api::reasoning::traits::Reasoning) implementation.

use crate::api::reasoning::types::ReasoningPattern;

/// Reference reasoner that executes a single linear chain-of-thought pass.
///
/// A domain primitive with no LLM backend: it produces deterministic steps and
/// conclusions so the [`Reasoning`](crate::api::reasoning::traits::Reasoning)
/// contract can be exercised deterministically in tests and wiring.
#[derive(Clone, Debug)]
pub struct LinearReasoning {
    pub(crate) pattern: ReasoningPattern,
}
