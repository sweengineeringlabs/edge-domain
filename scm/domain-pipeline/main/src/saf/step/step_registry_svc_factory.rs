//! Step registry service — opaque construction surface for [`StepRegistry`](crate::api::StepRegistry).

use std::sync::Arc;

use crate::api::StepRegistry;
use crate::core::traits::DefaultStepRegistry;

/// Identifies the step registry `Service` implementation at runtime.
pub const STEP_REGISTRY_SVC: &str = "step_registry";

/// Identifies the `StepRegistrySvc` factory implementation.
pub const STEP_REGISTRY_SVC_FACTORY: &str = "step_registry_svc_factory";

/// Construction handle for [`StepRegistry`](crate::api::StepRegistry) instances.
///
/// Consumers declare a dependency on `Box<dyn StepRegistry<Ctx>>` (exclusive ownership)
/// or `Arc<dyn StepRegistry<Ctx>>` (shared ownership) and obtain one through the
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
/// use edge_domain_pipeline::api::{PipelineDefinition, StepRegistry};
/// use edge_domain_pipeline::StepRegistrySvc;
///
/// let registry = StepRegistrySvc::create::<MyCtx>();
/// registry.register("enrich", Arc::new(EnrichStep));
/// registry.register("publish", Arc::new(PublishStep));
///
/// let definition = PipelineDefinition::load("pipeline.toml")?;
/// let pipeline = registry.build_pipeline(&definition)?;
/// pipeline.execute(&mut ctx).await?;
/// ```
///
/// ## Shared ownership
///
/// ```rust,ignore
/// use edge_domain_pipeline::StepRegistrySvc;
///
/// let registry = StepRegistrySvc::create_shared::<MyCtx>();
/// let registry_clone = Arc::clone(&registry);
/// ```
pub struct StepRegistrySvc;

impl StepRegistrySvc {
    /// Create an empty step registry with exclusive ownership.
    ///
    /// Steps are registered by name via [`StepRegistry::register`] before
    /// calling [`StepRegistry::build_pipeline`].
    pub fn create<Ctx: Send + 'static>() -> Box<dyn StepRegistry<Ctx>> {
        Box::new(DefaultStepRegistry::new())
    }

    /// Create an empty step registry with shared ownership.
    ///
    /// Use when the same registry instance must be shared across threads or services.
    pub fn create_shared<Ctx: Send + 'static>() -> Arc<dyn StepRegistry<Ctx>> {
        Arc::new(DefaultStepRegistry::new())
    }
}
