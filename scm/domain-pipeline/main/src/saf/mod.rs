//! Service Abstraction Framework — public API surface (traits only).

pub mod pipeline_svc;
pub mod step_svc;
pub mod validator_svc;

// Re-export through _svc modules
pub use pipeline_svc::{create_pipeline, create_pipeline_with_config, PIPELINE_SVC};
pub use pipeline_svc::{Pipeline, PipelineConfig, PipelineError};
pub use step_svc::{Step, STEP_SVC};
pub use validator_svc::{create_validator, Validator, VALIDATOR_SVC};

// Re-export test helpers
pub use crate::spi::{PipelineBuilder, NoopStep, AlwaysPassStep, AlwaysFailStep, MutatingStep};
