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

pub use api::{
    BuilderValidationRequest, ConfigValidationRequest, ContextMutationRequest, EnablementRequest,
    EnablementResponse, ParallelBranchFailure, ParallelConfig, ParallelExecutor,
    ParallelStepBuilder, ParallelStepError, Pipeline, PipelineAssemblyRequest,
    PipelineAssemblyResponse, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineConfigResponse, PipelineDefinition, PipelineEmptinessRequest,
    PipelineEmptinessResponse, PipelineError, Step, StepCountRequest, StepCountResponse, StepError,
    StepFailureRequest, StepFailureResponse, StepNameRequest, StepNameResponse,
    StepRegistrationRequest, StepRegistry, Validator,
};
pub use saf::{
    ParallelStepSvc, PipelineSvc, StepRegistrySvc, StepSvc, ValidatorSvc, PARALLEL_STEP_SVC,
    PARALLEL_STEP_SVC_FACTORY, PIPELINE_SVC, PIPELINE_SVC_FACTORY, STEP_REGISTRY_SVC,
    STEP_REGISTRY_SVC_FACTORY, STEP_SVC, STEP_SVC_FACTORY, VALIDATOR_SVC, VALIDATOR_SVC_FACTORY,
};
