//! # edge-llm-reasoning
//!
//! LLM Reasoning domain primitive: multi-strategy reasoning patterns
//! (chain-of-thought, tree-of-thought, reflection, …) for complex problem solving.
//!
//! Public surface is delegated entirely through `saf/`.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

// Re-export SAF layer traits and factory markers
pub use saf::{
    Reasoning, ReasoningBootstrap, REASONING_FACTORY_SVC, REASONING_SVC,
};

// Re-export API value types for integration tests and client libraries
pub use api::{
    ReasoningError,
    LinearReasoning, PatternMetadata, ReasoningChain,
    ReasoningPattern, ReasoningStep, StdReasoningFactory, StepResult,
    ThinkingProcess,
    PatternMetadataBuilder, ReasoningChainBuilder, ReasoningStepBuilder,
    StepResultBuilder, ThinkingProcessBuilder,
};
