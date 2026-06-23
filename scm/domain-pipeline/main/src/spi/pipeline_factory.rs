//! Factory for creating pipeline instances (implementation).

use std::sync::Arc;

use crate::api::{PipelineConfig, Pipeline, Step};
use crate::spi::default_pipeline::DefaultPipeline;

/// Internal factory for creating pipeline instances.
pub(crate) struct PipelineFactory;

impl PipelineFactory {
    /// Create a pipeline with the given steps and default config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub(crate) fn create<Ctx: Send + 'static>(steps: Vec<Arc<dyn Step<Ctx>>>) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::new(steps))
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub(crate) fn create_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        Box::new(DefaultPipeline::with_config(steps, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_happy_creates_pipeline() {
        let steps: Vec<Arc<dyn Step<i32>>> = vec![];
        let pipeline = PipelineFactory::create(steps);
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_create_with_config_happy_applies_config() {
        use std::time::Duration;

        let steps: Vec<Arc<dyn Step<i32>>> = vec![];
        let config = PipelineConfig {
            timeout_per_step: Some(Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline = PipelineFactory::create_with_config(steps, config);
        assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(5)));
    }
}

