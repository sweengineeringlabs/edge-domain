//! [`StepRegistry`] — maps step names to shared step instances for TOML-driven pipeline assembly.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineError, Step};
use crate::api::types::PipelineDefinition;

/// Maps step names to shared step instances for a given context type.
///
/// Populate at startup by registering concrete [`Step`] implementations under string names,
/// then assemble a pipeline from a [`PipelineDefinition`] loaded from TOML.
///
/// ## Example
/// ```rust,ignore
/// let mut registry = create_step_registry::<MyCtx>();
/// registry.register("validate", Arc::new(ValidateStep));
/// registry.register("enrich",   Arc::new(EnrichStep));
///
/// let definition = PipelineDefinition::load(&loader)?;
/// let pipeline   = registry.build_pipeline(&definition)?;
/// pipeline.execute(&mut ctx).await?;
/// ```
pub trait StepRegistry<Ctx>: Send + Sync {
    /// Register a step under `name`. Replaces any prior registration for the same name.
    fn register(&mut self, name: &str, step: Arc<dyn Step<Ctx>>);

    /// Assemble a pipeline by resolving each name in `definition.steps` from the registry.
    ///
    /// Returns [`PipelineError::UnknownStep`] on the first name not present in the registry.
    /// Returns an empty pipeline (succeeds immediately) when `definition.steps` is empty.
    fn build_pipeline(
        &self,
        definition: &PipelineDefinition,
    ) -> Result<Box<dyn Pipeline<Ctx>>, PipelineError>;
}
