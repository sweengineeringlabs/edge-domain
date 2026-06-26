//! [`StepRegistry`] — maps step names to shared step instances for TOML-driven assembly.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineError, Step, StepError};
use crate::api::types::PipelineDefinition;

/// Maps step names to shared step instances for a given context and error type.
///
/// Populate at startup by registering concrete [`Step`] implementations under string names,
/// then assemble a pipeline from a [`PipelineDefinition`] loaded from TOML.
///
/// ## Example
/// ```rust,ignore
/// let mut registry = StepRegistrySvc::create::<MyCtx, MyError>();
/// registry.register("validate", Arc::new(ValidateStep));
/// registry.register("enrich",   Arc::new(EnrichStep));
///
/// let definition = PipelineDefinition::load(&loader)?;
/// let pipeline   = registry.build_pipeline(&definition)?;
/// pipeline.run(&mut ctx).await?;
/// ```
pub trait StepRegistry: Send + Sync {
    /// The context type shared across all steps in this registry.
    type Ctx: Send + 'static;
    /// The domain error type returned by steps in this registry.
    type E: Send + 'static;

    /// Register a step under `name`. Replaces any prior registration for the same name.
    fn register(&mut self, name: &str, step: Arc<dyn Step<Self::Ctx, Self::E>>);

    /// Assemble a pipeline by resolving each name in `definition.steps` from the registry.
    ///
    /// Returns [`PipelineError::UnknownStep`] on the first name not present in the registry.
    /// Returns an empty pipeline (succeeds immediately) when `definition.steps` is empty.
    fn build_pipeline(
        &self,
        definition: &PipelineDefinition,
    ) -> Result<Box<dyn Pipeline<Self::Ctx, Self::E>>, PipelineError<Self::E>>;

    /// Wrap `cause` in a [`StepError`] annotated with the given step name.
    ///
    /// Diagnostic helper for consumers that build step errors without a live step reference.
    fn step_error_for(&self, step_name: &str, cause: Self::E) -> StepError<Self::E> {
        StepError {
            step_name: step_name.to_string(),
            cause,
        }
    }
}
