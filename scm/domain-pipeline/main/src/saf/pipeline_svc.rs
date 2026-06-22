//! Pipeline service facade — provides orchestration interface.

use std::sync::Arc;
use crate::api::{Step, PipelineConfig};
use crate::core::default::pipeline::DefaultPipeline;

// Re-export Pipeline trait from api
pub use crate::api::Pipeline;

// Re-export error and config types from api layer
pub use crate::api::{PipelineError as SvcPipelineError, PipelineConfig as SvcPipelineConfig};

/// Marker constant for pipeline service identification.
pub const PIPELINE_SVC: &str = "pipeline";

/// Create a pipeline with given steps and default config.
/// # Errors
/// This function delegates to [`DefaultPipeline::new`] which cannot fail.
pub fn create_pipeline<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::new(steps))
}

/// Create a pipeline with custom config.
/// # Errors
/// This function delegates to [`DefaultPipeline::with_config`] which cannot fail.
pub fn create_pipeline_with_config<Ctx: Send + 'static>(
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
) -> Box<dyn Pipeline<Ctx>> {
    Box::new(DefaultPipeline::with_config(steps, config))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_pipeline
    #[test]
    fn test_create_pipeline_happy_empty() {
        let pipeline = create_pipeline::<()>(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    /// @covers: create_pipeline
    #[tokio::test]
    async fn test_create_pipeline_happy_executes() {
        let pipeline = create_pipeline::<()>(vec![]);
        let mut ctx = ();
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }

    /// @covers: create_pipeline_with_config
    #[test]
    fn test_create_pipeline_with_config_happy_applies() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline = create_pipeline_with_config::<()>(vec![], config.clone());
        assert_eq!(pipeline.config().timeout_per_step, Some(std::time::Duration::from_secs(5)));
        assert!(pipeline.config().emit_lifecycle_events);
        assert!(!pipeline.config().abort_on_error);
    }

    /// @covers: create_pipeline_with_config
    #[tokio::test]
    async fn test_create_pipeline_with_config_happy_executes() {
        let config = PipelineConfig::default();
        let pipeline = create_pipeline_with_config::<()>(vec![], config);
        let mut ctx = ();
        assert!(pipeline.execute(&mut ctx).await.is_ok());
    }

    /// @covers: PIPELINE_SVC constant
    #[test]
    fn test_pipeline_svc_constant() {
        assert_eq!(PIPELINE_SVC, "pipeline");
    }
}
