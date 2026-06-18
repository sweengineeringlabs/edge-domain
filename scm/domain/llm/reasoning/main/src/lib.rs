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

pub use saf::{
    LinearReasoning, PatternMetadata, PatternMetadataBuilder, Reasoning, ReasoningChain,
    ReasoningChainBuilder, ReasoningError, ReasoningFactory, ReasoningPattern, ReasoningStep,
    ReasoningStepBuilder, StdReasoningFactory, StepResult, StepResultBuilder, ThinkingProcess,
    ThinkingProcessBuilder, REASONING_FACTORY_SVC, REASONING_SVC,
};
