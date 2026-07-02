//! [`Pipeline`] — orchestrates a sequence of steps.

use edge_domain_service::Service;

use crate::api::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineConfigResponse, PipelineEmptinessRequest, PipelineEmptinessResponse, PipelineError,
    StepCountRequest, StepCountResponse,
};

/// Orchestrates a sequence of [`Step`](crate::Step) operations.
///
/// `Pipeline` extends [`Service`] with `Request = Self::Ctx` and `Response = Self::Ctx`,
/// making every pipeline a first-class domain service. The dispatcher bridge
/// ([`edge_domain_handler::IntoHandler`]) fires automatically on any `Pipeline`
/// implementor — no wrapper required.
///
/// `Self::E` is the consumer's domain error type. All steps registered in this pipeline
/// must implement `Step<Ctx = Self::Ctx, ExecutionError = Self::E>`. The engine wraps step
/// errors in [`PipelineError::StepFailed`] with the step name added as context.
///
/// # Invariant
///
/// Steps execute sequentially. The pipeline is not parallel.
#[async_trait::async_trait]
pub trait Pipeline: Service<Request = Self::Ctx, Response = Self::Ctx> {
    /// The context type threaded through every step.
    type Ctx: Send + 'static;
    /// The consumer's domain error type.
    type E: Send + 'static;

    /// Run all steps in order, passing a mutable context through each.
    ///
    /// On the first step error the engine wraps it in
    /// [`PipelineError::StepFailed`] and halts (unless `abort_on_error = false`).
    ///
    /// # Errors
    ///
    /// Returns the first [`PipelineError<E>`] encountered. The context may be
    /// partially mutated from earlier steps.
    async fn run(
        &self,
        req: ContextMutationRequest<'_, Self::Ctx>,
    ) -> Result<(), PipelineError<Self::E>>;

    /// Return the number of steps in this pipeline.
    fn step_count(
        &self,
        req: StepCountRequest,
    ) -> Result<StepCountResponse, PipelineError<Self::E>>;

    /// Return true if the pipeline has no steps.
    fn is_empty(
        &self,
        req: PipelineEmptinessRequest,
    ) -> Result<PipelineEmptinessResponse, PipelineError<Self::E>> {
        let _ = req;
        Ok(PipelineEmptinessResponse {
            empty: self.step_count(StepCountRequest)?.count == 0,
        })
    }

    /// Get the pipeline configuration.
    fn config(
        &self,
        req: PipelineConfigLookupRequest,
    ) -> Result<PipelineConfigResponse, PipelineError<Self::E>>;

    /// Create a new fluent builder for assembling a pipeline.
    fn new_builder() -> PipelineBuilder<Self::Ctx, Self::E>
    where
        Self: Sized,
    {
        PipelineBuilder {
            steps: Vec::new(),
            config: PipelineConfig::default(),
            event_bus: None,
        }
    }
}
