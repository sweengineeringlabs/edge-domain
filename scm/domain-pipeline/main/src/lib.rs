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
pub use api::{PipelineConfig, PipelineError, Pipeline, Step, Validator, PipelineFactory, ValidatorFactory};
pub use saf::{PIPELINE_SVC, STEP_SVC, VALIDATOR_SVC};
pub use saf::pipeline_svc::{create_pipeline, create_pipeline_with_config};
pub use saf::validator_svc::create_validator;
