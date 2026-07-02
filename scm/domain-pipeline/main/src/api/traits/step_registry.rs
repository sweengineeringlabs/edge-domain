//! [`StepRegistry`] — maps step names to shared step instances for TOML-driven assembly.

use crate::api::{
    PipelineAssemblyRequest, PipelineAssemblyResponse, PipelineError, StepFailureRequest,
    StepFailureResponse, StepRegistrationRequest,
};

/// Maps step names to shared step instances for a given context and error type.
///
/// Populate at startup by registering concrete [`Step`](crate::Step) implementations under
/// string names, then assemble a pipeline from a
/// [`PipelineDefinition`](crate::PipelineDefinition) loaded from TOML.
///
/// ## Example
/// ```rust,ignore
/// let mut registry = StepRegistrySvc::create::<MyCtx, MyError>();
/// registry.register(StepRegistrationRequest { name: "validate".to_string(), step: Arc::new(ValidateStep) })?;
/// registry.register(StepRegistrationRequest { name: "enrich".to_string(), step: Arc::new(EnrichStep) })?;
///
/// let definition = PipelineDefinition::load(&loader)?;
/// let pipeline   = registry.build_pipeline(PipelineAssemblyRequest { definition })?.pipeline;
/// pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await?;
/// ```
pub trait StepRegistry: Send + Sync {
    /// The context type shared across all steps in this registry.
    type Ctx: Send + 'static;
    /// The domain error type returned by steps in this registry.
    type E: Send + 'static;

    /// Register a step under `req.name`. Replaces any prior registration for the same name.
    fn register(
        &mut self,
        req: StepRegistrationRequest<Self::Ctx, Self::E>,
    ) -> Result<(), PipelineError<Self::E>>;

    /// Assemble a pipeline by resolving each name in `req.definition.steps` from the registry.
    ///
    /// Returns [`PipelineError::UnknownStep`] on the first name not present in the registry.
    /// Returns an empty pipeline (succeeds immediately) when `req.definition.steps` is empty.
    ///
    /// The return type is spelled out literally (rather than via the `PipelineBuildResult`
    /// alias) so the arch structural checker can see the named `*Response`/`*Error` shape.
    #[allow(clippy::type_complexity)]
    fn build_pipeline(
        &self,
        req: PipelineAssemblyRequest,
    ) -> Result<PipelineAssemblyResponse<Self::Ctx, Self::E>, PipelineError<Self::E>>;

    /// Wrap `req.cause` in a [`StepError`](crate::StepError) annotated with `req.step_name`.
    ///
    /// Diagnostic helper for consumers that build step errors without a live step reference.
    fn step_error_for(
        &self,
        req: StepFailureRequest<Self::E>,
    ) -> Result<StepFailureResponse<Self::E>, PipelineError<Self::E>> {
        Ok(StepFailureResponse {
            error: crate::api::StepError {
                step_name: req.step_name,
                cause: req.cause,
            },
        })
    }
}
