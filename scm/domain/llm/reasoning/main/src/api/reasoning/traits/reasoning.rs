//! `Reasoning` — the multi-strategy reasoning contract (primary trait).

use async_trait::async_trait;

use crate::api::reasoning::errors::ReasoningError;
use crate::api::reasoning::types::{
    ChainBuildRequest, ChainBuildResponse, NextStepRequest, NextStepResponse,
    PatternMetadataLookupRequest, PatternMetadataLookupResponse, PatternSupportRequest,
    PatternSupportResponse, ProblemValidationRequest, ReasonRequest, ReasonResponse,
    StepEvaluationRequest, StepEvaluationResponse, SupportedPatternsRequest,
    SupportedPatternsResponse,
};

/// Orchestrates reasoning processes for a given pattern.
///
/// Decouples callers from any specific reasoning strategy: they depend on this
/// contract and inject a concrete reasoner (chain-of-thought, tree-of-thought, …).
#[async_trait]
pub trait Reasoning: Send + Sync {
    /// Execute reasoning for the requested problem and pattern.
    ///
    /// Returns [`ReasoningError`] when the pattern is unsupported or a step fails.
    async fn reason(&self, req: ReasonRequest<'_>) -> Result<ReasonResponse, ReasoningError>;

    /// Patterns this reasoner can execute.
    fn supported_patterns(
        &self,
        req: SupportedPatternsRequest,
    ) -> Result<SupportedPatternsResponse, ReasoningError>;

    /// Whether the requested pattern is supported.
    fn supports_pattern(
        &self,
        req: PatternSupportRequest,
    ) -> Result<PatternSupportResponse, ReasoningError> {
        let supported = self
            .supported_patterns(SupportedPatternsRequest)?
            .patterns
            .contains(&req.pattern);
        Ok(PatternSupportResponse { supported })
    }

    /// Configuration metadata for the requested pattern, if supported.
    fn pattern_metadata(
        &self,
        req: PatternMetadataLookupRequest,
    ) -> Result<PatternMetadataLookupResponse, ReasoningError>;

    /// Validate a reasoning problem before execution.
    ///
    /// Returns [`ReasoningError::InvalidState`] when the problem is unusable.
    fn validate_problem(&self, req: ProblemValidationRequest<'_>) -> Result<(), ReasoningError>;

    /// Produce the next reasoning step for an in-progress process.
    fn next_step(&self, req: NextStepRequest<'_>) -> Result<NextStepResponse, ReasoningError>;

    /// Evaluate a completed reasoning step.
    fn evaluate_step(
        &self,
        req: StepEvaluationRequest<'_>,
    ) -> Result<StepEvaluationResponse, ReasoningError>;

    /// Assemble an ordered set of processes into a reasoning chain.
    fn build_chain(&self, req: ChainBuildRequest<'_>)
        -> Result<ChainBuildResponse, ReasoningError>;
}
