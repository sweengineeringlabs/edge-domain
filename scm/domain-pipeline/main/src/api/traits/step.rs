//! [`Step`] — single composable operation in a pipeline.

use crate::api::{
    ContextMutationRequest, PipelineError, StepFailureRequest, StepFailureResponse,
    StepNameRequest, StepNameResponse,
};

/// A single composable step in a [`Pipeline`](crate::api::Pipeline).
///
/// Steps execute sequentially, each receiving a mutable reference to shared context.
/// On success (`Ok(())`), context mutations are applied and execution continues.
/// On failure (`Err(Self::ExecutionError)`), the engine wraps the error in a
/// [`StepError<E>`](crate::StepError) (adding the step name) and either aborts
/// or continues depending on `abort_on_error` config.
///
/// `Self::ExecutionError` is the consumer's domain error type. All steps in one pipeline must
/// share it.
///
/// # Invariant
///
/// A step must be idempotent over the context — calling it twice with the same context
/// must produce the same result or error.
#[async_trait::async_trait]
pub trait Step: Send + Sync {
    /// The context type this step mutates.
    type Ctx;
    /// The consumer's domain error type.
    type ExecutionError: Send + 'static;

    /// Execute this step, mutating the context in-place.
    ///
    /// Returns `Ok(())` on success or `Err(Self::ExecutionError)` on failure.
    /// The context may be partially mutated before an error; callers must not assume rollback.
    async fn execute(
        &self,
        req: ContextMutationRequest<'_, Self::Ctx>,
    ) -> Result<(), Self::ExecutionError>;

    /// Human-readable name for this step (logging, debugging, observability).
    fn name(
        &self,
        req: StepNameRequest,
    ) -> Result<StepNameResponse, PipelineError<Self::ExecutionError>> {
        let _ = req;
        Ok(StepNameResponse {
            name: std::any::type_name::<Self>().to_string(),
        })
    }

    /// Wrap `req.cause` in a [`StepError`](crate::StepError) annotated with `req.step_name`.
    ///
    /// Convenience for steps that need to attach context before returning `Err`.
    fn fail_with(
        &self,
        req: StepFailureRequest<Self::ExecutionError>,
    ) -> Result<StepFailureResponse<Self::ExecutionError>, PipelineError<Self::ExecutionError>>
    {
        Ok(StepFailureResponse {
            error: crate::api::StepError {
                step_name: req.step_name,
                cause: req.cause,
            },
        })
    }
}
