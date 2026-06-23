//! Reasoning domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ReasoningError;
pub use traits::{Reasoning, ReasoningBootstrap};
pub use types::{
    LinearReasoning, PatternMetadata, ReasoningChain,
    ReasoningPattern, ReasoningStep, StdReasoningFactory, StepResult,
    ThinkingProcess,
    PatternMetadataBuilder, ReasoningChainBuilder, ReasoningStepBuilder,
    StepResultBuilder, ThinkingProcessBuilder,
};
