//! Reasoning domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub(crate) mod pattern;
pub(crate) mod step;
pub(crate) mod thinking;

pub use errors::ReasoningError;
pub use traits::{Reasoning, ReasoningBootstrap};
pub use types::{
    ChainBuildRequest, ChainBuildResponse, LinearReasoning, NextStepRequest, NextStepResponse,
    PatternMetadata, PatternMetadataBuilder, PatternMetadataLookupRequest,
    PatternMetadataLookupResponse, PatternSupportRequest, PatternSupportResponse,
    ProblemValidationRequest, ReasonRequest, ReasonResponse, ReasoningBootstrapNameRequest,
    ReasoningBootstrapNameResponse, ReasoningChain, ReasoningChainBuilder, ReasoningPattern,
    ReasoningStep, ReasoningStepBuilder, StdReasoningFactory, StepEvaluationRequest,
    StepEvaluationResponse, StepResult, StepResultBuilder, SupportedPatternsRequest,
    SupportedPatternsResponse, ThinkingProcess, ThinkingProcessBuilder,
};
