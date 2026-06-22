//! [`PipelineAsStep<Ctx>`] — adapter wrapper for using pipelines as steps.

use crate::api::{Step, Pipeline, PipelineError};

/// Adapter wrapper that allows a Pipeline to be used as a Step.
/// This is useful for nested pipelines and composition.
pub struct PipelineAsStep<Ctx: Send + 'static> {
    pipeline: Box<dyn Pipeline<Ctx>>,
}

impl<Ctx: Send + 'static> PipelineAsStep<Ctx> {
    /// Wrap a pipeline so it can be used as a step.
    pub fn new(pipeline: Box<dyn Pipeline<Ctx>>) -> Self {
        Self { pipeline }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send + 'static> Step<Ctx> for PipelineAsStep<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        self.pipeline.execute(ctx).await
    }

    fn name(&self) -> &str {
        "pipeline-as-step"
    }
}
