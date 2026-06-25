//! Pipeline service — opaque construction surface for [`Pipeline`](crate::api::Pipeline).

use std::sync::Arc;

use edge_domain_service::Service;

use crate::api::{Pipeline, PipelineBuilder};
use crate::core::pipeline::DefaultPipeline;

/// Identifies the pipeline `Service` implementation at runtime.
pub const PIPELINE_SVC: &str = "pipeline";

/// Identifies the `PipelineSvc` factory implementation.
pub const PIPELINE_SVC_FACTORY: &str = "pipeline_svc_factory";

/// Construction handle for [`Pipeline`](crate::api::Pipeline) instances.
///
/// Consumers declare a dependency on `Box<dyn Pipeline<Ctx>>` (exclusive ownership),
/// `Arc<dyn Pipeline<Ctx>>` (shared ownership), or an opaque `impl Service` for
/// dispatcher integration via [`as_service`](PipelineSvc::as_service).
/// The concrete implementation (`DefaultPipeline`) is never exposed.
///
/// # Examples
///
/// ## Exclusive ownership
///
/// ```rust,ignore
/// use edge_domain_pipeline::{PipelineSvc, PipelineBuilder};
///
/// let pipeline = PipelineSvc::build(
///     PipelineBuilder::new()
///         .with(EnrichStep)
///         .abort_on_error(true),
/// );
/// pipeline.execute(&mut ctx).await?;
/// ```
///
/// ## Dispatcher integration (no wrapper)
///
/// ```rust,ignore
/// use edge_domain_pipeline::{PipelineSvc, PipelineBuilder};
/// use edge_domain_handler::IntoHandler;
///
/// let handler = PipelineSvc::as_service(PipelineBuilder::new().with(EnrichStep))
///     .into_handler();
/// registry.register(Arc::new(handler));
/// ```
pub struct PipelineSvc;

impl PipelineSvc {
    /// Build a pipeline with exclusive ownership.
    pub fn build<Ctx: Send + 'static>(
        builder: PipelineBuilder<Ctx>,
    ) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::with_config(builder.steps, builder.config))
    }

    /// Build a pipeline with shared ownership.
    pub fn build_shared<Ctx: Send + 'static>(
        builder: PipelineBuilder<Ctx>,
    ) -> Arc<dyn Pipeline<Ctx>> {
        Arc::new(DefaultPipeline::with_config(builder.steps, builder.config))
    }

    /// Return an opaque [`Service`] over this pipeline for dispatcher integration.
    ///
    /// Callers chain `.into_handler()` (from `edge_domain_handler::IntoHandler`) to
    /// register the pipeline in a `HandlerRegistry` without any additional wrapper.
    pub fn as_service<Ctx: Send + 'static>(
        builder: PipelineBuilder<Ctx>,
    ) -> impl Service<Request = Ctx, Response = Ctx> {
        DefaultPipeline::with_config(builder.steps, builder.config)
    }
}
