//! Step registry service — opaque construction surface for [`StepRegistry`](crate::api::StepRegistry).

use std::fmt;
use std::sync::Arc;

use crate::api::StepRegistry;
use crate::core::traits::DefaultStepRegistry;

/// Identifies the step registry `Service` implementation at runtime.
pub const STEP_REGISTRY_SVC: &str = "step_registry";

/// Identifies the `StepRegistrySvc` factory implementation.
pub const STEP_REGISTRY_SVC_FACTORY: &str = "step_registry_svc_factory";

/// Construction handle for [`StepRegistry`](crate::api::StepRegistry) instances.
///
/// Consumers declare a dependency on `Box<dyn StepRegistry<Ctx, E>>` (exclusive ownership)
/// or `Arc<dyn StepRegistry<Ctx, E>>` (shared ownership) and obtain one through the
/// corresponding method. Steps are registered by name and later resolved when
/// building a pipeline from a [`PipelineDefinition`](crate::api::PipelineDefinition).
/// The concrete implementation (`DefaultStepRegistry`) is never exposed.
///
/// # Examples
///
/// ## Exclusive ownership
///
/// ```rust,ignore
/// use std::sync::Arc;
/// use edge_domain_pipeline::{PipelineDefinition, StepRegistrySvc};
///
/// let mut registry = StepRegistrySvc::create::<MyCtx, MyError>();
/// registry.register("enrich", Arc::new(EnrichStep));
/// registry.register("publish", Arc::new(PublishStep));
///
/// let definition = PipelineDefinition::load("pipeline.toml")?;
/// let pipeline = registry.build_pipeline(&definition)?;
/// pipeline.run(&mut ctx).await?;
/// ```
pub struct StepRegistrySvc;

impl StepRegistrySvc {
    /// Create an empty step registry with exclusive ownership.
    pub fn create<Ctx, E>() -> Box<dyn StepRegistry<Ctx = Ctx, E = E>>
    where
        Ctx: Send + 'static,
        E: fmt::Display + fmt::Debug + Send + 'static,
    {
        Box::new(DefaultStepRegistry::new())
    }

    /// Create an empty step registry with shared ownership.
    pub fn create_shared<Ctx, E>() -> Arc<dyn StepRegistry<Ctx = Ctx, E = E>>
    where
        Ctx: Send + 'static,
        E: fmt::Display + fmt::Debug + Send + 'static,
    {
        Arc::new(DefaultStepRegistry::new())
    }
}
