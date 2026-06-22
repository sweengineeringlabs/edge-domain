//! Service Abstraction Framework — public API surface and factory functions.
//!
//! Provides factory functions for creating domain instances.
//! Traits are re-exported in lib.rs.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineConfig, Step, Validator};
use crate::core::default::pipeline::DefaultPipeline;
use crate::spi::config_validator::ConfigValidator;

/// Service name constant for pipeline service.
pub const PIPELINE_SVC: &str = "pipeline";

/// Service name constant for step service.
pub const STEP_SVC: &str = "step";

/// Service name constant for validator service.
pub const VALIDATOR_SVC: &str = "validator";

/// Create a pipeline with the given steps and default config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::new(steps))
}

/// Create a pipeline with the given steps and custom config.
///
/// Returns an opaque trait object so callers never see concrete implementation types.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(steps, config))
}

/// Create a config validator strategy.
///
/// # Arguments
/// * `enabled` - Whether the validator should enforce validation rules
///
/// # Returns
/// A boxed validator instance
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    Box::new(ConfigValidator::new(enabled))
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

    /// @covers: create_validator
    #[test]
    fn test_create_validator_happy_enabled() {
        let validator = create_validator(true);
        assert!(validator.is_enabled());
    }

    /// @covers: create_validator
    #[test]
    fn test_create_validator_happy_disabled() {
        let validator = create_validator(false);
        assert!(!validator.is_enabled());
    }
}
