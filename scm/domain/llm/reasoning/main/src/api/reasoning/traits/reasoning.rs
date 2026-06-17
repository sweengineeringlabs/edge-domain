//! `Reasoning` — the multi-strategy reasoning contract (primary trait).

use async_trait::async_trait;

use crate::api::reasoning::errors::ReasoningError;
use crate::api::reasoning::types::{
    PatternMetadata, ReasoningChain, ReasoningPattern, ReasoningStep, StepResult, ThinkingProcess,
};

/// Orchestrates reasoning processes for a given pattern.
///
/// Decouples callers from any specific reasoning strategy: they depend on this
/// contract and inject a concrete reasoner (chain-of-thought, tree-of-thought, …).
#[async_trait]
pub trait Reasoning: Send + Sync {
    /// Execute reasoning for `problem` using the requested `pattern`.
    ///
    /// Returns [`ReasoningError`] when the pattern is unsupported or a step fails.
    async fn reason(
        &self,
        problem: &str,
        pattern: ReasoningPattern,
    ) -> Result<ThinkingProcess, ReasoningError>;

    /// Patterns this reasoner can execute.
    fn supported_patterns(&self) -> Vec<ReasoningPattern>;

    /// Whether `pattern` is supported.
    fn supports_pattern(&self, pattern: ReasoningPattern) -> bool {
        self.supported_patterns().contains(&pattern)
    }

    /// Configuration metadata for `pattern`, if supported.
    fn pattern_metadata(&self, pattern: ReasoningPattern) -> Option<PatternMetadata>;

    /// Validate a reasoning problem before execution.
    ///
    /// Returns [`ReasoningError::InvalidState`] when the problem is unusable.
    fn validate_problem(&self, problem: &str) -> Result<(), ReasoningError>;

    /// Produce the next reasoning step for an in-progress `process`.
    fn next_step(&self, process: &ThinkingProcess) -> ReasoningStep;

    /// Evaluate a completed reasoning `step`.
    fn evaluate_step(&self, step: &ReasoningStep) -> StepResult;

    /// Assemble an ordered set of `processes` into a reasoning chain.
    fn build_chain(&self, chain_id: &str, processes: Vec<ThinkingProcess>) -> ReasoningChain;
}
