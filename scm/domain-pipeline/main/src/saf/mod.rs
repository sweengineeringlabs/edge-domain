//! Service Abstraction Framework — public API surface (traits only).

mod pipeline_svc;
mod step_svc;
mod validator_svc;

// Re-export traits and types from api
pub use crate::api::{Pipeline, PipelineConfig, PipelineError, Step, Validator};

// Re-export factories
pub use pipeline_svc::{create_pipeline, create_pipeline_with_config, PIPELINE_SVC};
pub use step_svc::STEP_SVC;
pub use validator_svc::{create_validator, VALIDATOR_SVC};

// Re-export test helpers
pub use crate::spi::{PipelineBuilder, NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};
