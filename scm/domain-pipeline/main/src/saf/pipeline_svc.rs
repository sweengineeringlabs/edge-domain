//! Pipeline service factory — creates pipeline instances.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineConfig, Step};
use crate::spi;

/// Create a pipeline with the given steps and default config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(spi::DefaultPipeline::new(steps))
}

/// Create a pipeline with the given steps and custom config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(spi::DefaultPipeline::with_config(steps, config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spi::noop::AlwaysPassStep;

    /// @covers: create_pipeline
    #[tokio::test]
    async fn test_create_pipeline_happy_returns_pipeline() {
        let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(AlwaysPassStep::new())];
        let pipeline = create_pipeline(steps);
        let mut ctx = 0;
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }

    /// @covers: create_pipeline_with_config
    #[tokio::test]
    async fn test_create_pipeline_with_config_happy_uses_config() {
        let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(AlwaysPassStep::new())];
        let config = PipelineConfig::default();
        let pipeline = create_pipeline_with_config(steps, config);
        let mut ctx = 0;
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }
}
