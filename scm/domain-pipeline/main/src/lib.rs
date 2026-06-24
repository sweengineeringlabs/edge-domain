//! edge-domain — Pipeline port contract: immediate/synchronous step-chain orchestration
//!
//! Provides a generic pattern for composing ordered steps that mutate shared context.
//! Unlike Handler (async with branching) or Saga (event-driven, long-running),
//! Pipeline is for immediate, linear execution: step → step → step with fail-fast error handling.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

#[cfg(test)]
mod test_steps;

pub use api::{PipelineBuilder, PipelineConfig, PipelineDefinition, PipelineError};
pub use saf::{Pipeline, Step, StepRegistry, Validator, PIPELINE_SVC, STEP_REGISTRY_SVC, STEP_SVC, VALIDATOR_SVC};

// ── Public construction surface ───────────────────────────────────────────────
// Wiring layer: constructs core implementations and returns opaque trait objects.
// Lives here (crate root) because it must cross api/ → core/ direction, which no
// SEA layer may do — the crate boundary is the only legal wiring point.

use std::sync::Arc;
use core::pipeline::DefaultPipeline;
use core::traits::{DefaultStepRegistry, DefaultValidator};

/// Create a pipeline with default configuration.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::new(steps))
}

/// Create a pipeline with custom configuration.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(steps, config))
}

/// Create a validator with the given enabled state.
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    Box::new(DefaultValidator::new(enabled))
}

/// Build a pipeline from a completed [`PipelineBuilder`].
///
/// This is the terminal step of the builder pattern:
/// ```rust,ignore
/// let pipeline = build_pipeline(
///     PipelineBuilder::new()
///         .with(my_step)
///         .with_timeout(Duration::from_secs(5))
/// );
/// ```
pub fn build_pipeline<Ctx: Send + 'static>(builder: PipelineBuilder<Ctx>) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(builder.steps, builder.config))
}

/// Create a [`StepRegistry`] for assembling TOML-defined pipelines.
///
/// Register steps by name, then call
/// [`build_pipeline`](StepRegistry::build_pipeline) with a [`PipelineDefinition`](crate::PipelineDefinition)
/// loaded from TOML.
pub fn create_step_registry<Ctx: Send + 'static>() -> Box<dyn StepRegistry<Ctx>> {
    Box::new(DefaultStepRegistry::new())
}
