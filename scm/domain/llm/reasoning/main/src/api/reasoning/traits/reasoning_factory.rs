//! `ReasoningFactory` — constructor contract for the default reasoning primitives.

use std::sync::Arc;

use crate::api::reasoning::types::{
    LinearReasoning, PatternMetadataBuilder, ReasoningChainBuilder, ReasoningEndpoint,
    ReasoningPattern, ReasoningStepBuilder, StdReasoningFactory, StepResultBuilder,
    ThinkingProcessBuilder,
};

/// Factory for the standard reference reasoning implementations.
///
/// Implement on any unit struct to gain the standard constructors and builders.
pub trait ReasoningFactory {
    /// Return the standard reasoning-factory instance.
    fn std_factory() -> StdReasoningFactory {
        StdReasoningFactory
    }

    /// Construct the reference [`LinearReasoning`] for the given pattern.
    fn reasoning(pattern: ReasoningPattern) -> LinearReasoning {
        LinearReasoning::new(pattern)
    }

    /// Start a fluent [`ReasoningStepBuilder`] for the given step index.
    fn reasoning_step_builder(index: usize) -> ReasoningStepBuilder {
        ReasoningStepBuilder::new(index)
    }

    /// Start a fluent [`StepResultBuilder`].
    fn step_result_builder() -> StepResultBuilder {
        StepResultBuilder::new()
    }

    /// Start a fluent [`ThinkingProcessBuilder`] for the given process id.
    fn thinking_process_builder(id: String) -> ThinkingProcessBuilder {
        ThinkingProcessBuilder::new(id)
    }

    /// Start a fluent [`PatternMetadataBuilder`] for the given pattern.
    fn pattern_metadata_builder(pattern: ReasoningPattern) -> PatternMetadataBuilder {
        PatternMetadataBuilder::new(pattern)
    }

    /// Start a fluent [`ReasoningChainBuilder`] for the given chain id.
    fn reasoning_chain_builder(id: String) -> ReasoningChainBuilder {
        ReasoningChainBuilder::new(id)
    }

    /// Construct a dispatchable [`ReasoningEndpoint`] backed by a reference reasoner.
    fn endpoint(pattern: ReasoningPattern) -> ReasoningEndpoint {
        ReasoningEndpoint::new(Arc::new(LinearReasoning::new(pattern)))
    }
}
