//! Public trait contracts, types, and error types.

mod error;
mod traits;
mod types;
mod validator;

pub use error::{ParallelBranchFailure, ParallelStepError, PipelineError, StepError};
pub use traits::{ParallelExecutor, Pipeline, Step, StepRegistry, Validator};
pub use types::{
    BuilderValidationRequest, ConfigValidationRequest, ContextMutationRequest, EnablementRequest,
    EnablementResponse, ParallelConfig, ParallelStepBuilder, PipelineAssemblyRequest,
    PipelineAssemblyResponse, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineConfigResponse, PipelineDefinition, PipelineEmptinessRequest,
    PipelineEmptinessResponse, StepCountRequest, StepCountResponse, StepFailureRequest,
    StepFailureResponse, StepNameRequest, StepNameResponse, StepRegistrationRequest,
};
