//! `ReasoningBootstrap` — constructor contract for the default reasoning primitives.

use crate::api::reasoning::types::{
    LinearReasoning, PatternMetadataBuilder, ReasoningChainBuilder, ReasoningPattern,
    ReasoningStepBuilder, StdReasoningFactory, StepResultBuilder, ThinkingProcessBuilder,
};

/// Factory for the standard reference reasoning implementations.
///
/// Implement on any unit struct to gain the standard constructors and builders.
pub trait ReasoningBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "reasoning"
    }

    /// Return the standard reasoning-factory instance.
    fn std_factory() -> StdReasoningFactory where Self: Sized {
        StdReasoningFactory
    }

    /// Construct the reference [`LinearReasoning`] for the given pattern.
    fn reasoning(pattern: ReasoningPattern) -> LinearReasoning where Self: Sized {
        LinearReasoning::new(pattern)
    }

    /// Start a fluent [`ReasoningStepBuilder`] for the given step index.
    fn reasoning_step_builder(index: usize) -> ReasoningStepBuilder where Self: Sized {
        ReasoningStepBuilder::new(index)
    }

    /// Start a fluent [`StepResultBuilder`].
    fn step_result_builder() -> StepResultBuilder where Self: Sized {
        StepResultBuilder::new()
    }

    /// Start a fluent [`ThinkingProcessBuilder`] for the given process id.
    fn thinking_process_builder(id: String) -> ThinkingProcessBuilder where Self: Sized {
        ThinkingProcessBuilder::new(id)
    }

    /// Start a fluent [`PatternMetadataBuilder`] for the given pattern.
    fn pattern_metadata_builder(pattern: ReasoningPattern) -> PatternMetadataBuilder where Self: Sized {
        PatternMetadataBuilder::new(pattern)
    }

    /// Start a fluent [`ReasoningChainBuilder`] for the given chain id.
    fn reasoning_chain_builder(id: String) -> ReasoningChainBuilder where Self: Sized {
        ReasoningChainBuilder::new(id)
    }
}
