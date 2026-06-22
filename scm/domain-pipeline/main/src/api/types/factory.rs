//! Factory types for creating domain instances.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineConfig, Step, Validator};
use crate::spi;

/// Creates pipeline instances with configurable options.
pub struct PipelineFactory;

impl PipelineFactory {
    /// Create a pipeline with the given steps and default config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create<Ctx: Send + 'static>(steps: Vec<Arc<dyn Step<Ctx>>>) -> Box<dyn Pipeline<Ctx>> {
        Box::new(spi::DefaultPipeline::new(steps))
    }

    /// Create a pipeline with the given steps and custom config.
    ///
    /// Returns an opaque trait object so callers never see concrete implementation types.
    pub fn create_with_config<Ctx: Send + 'static>(
        steps: Vec<Arc<dyn Step<Ctx>>>,
        config: PipelineConfig,
    ) -> Box<dyn Pipeline<Ctx>> {
        Box::new(spi::DefaultPipeline::with_config(steps, config))
    }
}

/// Creates validator instances with configurable options.
pub struct ValidatorFactory;

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// # Arguments
    /// * `enabled` - Whether the validator should enforce validation rules
    ///
    /// # Returns
    /// A boxed validator instance
    pub fn create(enabled: bool) -> Box<dyn Validator> {
        Box::new(spi::ConfigValidator::new(enabled))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spi::noop::AlwaysPassStep;

    /// @covers PipelineFactory create
    #[tokio::test]
    async fn test_pipeline_factory_create_happy_returns_pipeline() {
        let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(AlwaysPassStep::new())];
        let pipeline = PipelineFactory::create(steps);
        let mut ctx = 0;
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }

    /// @covers PipelineFactory create_with_config
    #[tokio::test]
    async fn test_pipeline_factory_create_with_config_happy_uses_config() {
        let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(AlwaysPassStep::new())];
        let config = PipelineConfig::default();
        let pipeline = PipelineFactory::create_with_config(steps, config);
        let mut ctx = 0;
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }

    /// @covers ValidatorFactory create
    #[test]
    fn test_validator_factory_create_happy_enabled() {
        let validator = ValidatorFactory::create(true);
        assert!(validator.is_enabled());
    }

    /// @covers ValidatorFactory create
    #[test]
    fn test_validator_factory_create_happy_disabled() {
        let validator = ValidatorFactory::create(false);
        assert!(!validator.is_enabled());
    }
}
