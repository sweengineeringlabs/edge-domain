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
pub use api::{PipelineConfig, PipelineError, Pipeline, Step, Validator};
pub use spi::{PipelineFactory, ValidatorFactory};
pub use saf::{PIPELINE_SVC, STEP_SVC, VALIDATOR_SVC};
pub use saf::pipeline_svc::PipelineService;
pub use saf::validator_svc::ValidatorService;

use std::sync::Arc;

/// Create a pipeline with the given steps and default config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
///
/// This is a convenience wrapper around [`PipelineService::create_pipeline`].
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    PipelineService::create_pipeline(steps)
}

/// Create a pipeline with the given steps and custom config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
///
/// This is a convenience wrapper around [`PipelineService::create_pipeline_with_config`].
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    PipelineService::create_pipeline_with_config(steps, config)
}

/// Create a config validator strategy.
///
/// This is a convenience wrapper around [`ValidatorService::create_validator`].
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    ValidatorService::create_validator(enabled)
}
