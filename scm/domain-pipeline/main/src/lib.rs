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
mod spi;

// Public API surface
pub use api::{PipelineConfig, PipelineError, Pipeline, Step, Validator, PipelineService, ValidatorService};
pub use saf::{PIPELINE_SVC, STEP_SVC, VALIDATOR_SVC};

// Re-export factory facades
pub use saf::pipeline_svc::PipelineFactory;
pub use saf::validator_svc::ValidatorFactory;

// Convenience functions - these are thin wrappers for the factory methods
// placed at the crate root for API ergonomics
/// Create a pipeline with the given steps and default config.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<std::sync::Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    saf::pipeline_svc::PipelineFactory::create(steps)
}

/// Create a pipeline with the given steps and custom config.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<std::sync::Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    saf::pipeline_svc::PipelineFactory::create_with_config(steps, config)
}

/// Create a config validator strategy.
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    saf::validator_svc::ValidatorFactory::create(enabled)
}
