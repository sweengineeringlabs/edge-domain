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

impl LinearReasoning {
    /// Construct a reasoner bound to the given pattern.
    pub fn new(pattern: ReasoningPattern) -> Self {
        Self { pattern }
    }

    /// Pattern this reasoner executes.
    pub fn pattern(&self) -> ReasoningPattern {
        self.pattern
    }
}
