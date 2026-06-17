//! Reasoning domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ReasoningError;
pub use traits::{Reasoning, ReasoningFactory};
pub use types::{
    LinearReasoning, PatternMetadata, PatternMetadataBuilder, ReasoningChain,
    ReasoningChainBuilder, ReasoningPattern, ReasoningStep, ReasoningStepBuilder,
    StdReasoningFactory, StepResult, StepResultBuilder, ThinkingProcess, ThinkingProcessBuilder,
};
