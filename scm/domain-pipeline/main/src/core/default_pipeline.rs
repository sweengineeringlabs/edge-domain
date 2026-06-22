//! [`DefaultPipeline<Ctx>`] — orchestrates sequential step execution.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{Pipeline, PipelineError, Step};

/// Executes a sequence of steps in order, passing context through each.
#[derive(Clone)]
pub struct DefaultPipeline<Ctx> {
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
}

/// Configuration for pipeline execution.
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    /// Per-step timeout (optional).
    pub timeout_per_step: Option<Duration>,

    /// Emit lifecycle events (StepStarted, StepCompleted, etc).
    pub emit_lifecycle_events: bool,

    /// Abort on error (default: true). If false, silently skip failed steps.
    pub abort_on_error: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: true,
        }
    }
}

impl<Ctx: Send> DefaultPipeline<Ctx> {
    /// Create a new pipeline with given steps and default config.
    pub fn new(steps: Vec<Arc<dyn Step<Ctx>>>) -> Self {
        Self {
            steps,
            config: PipelineConfig::default(),
        }
    }

    /// Create a new pipeline with custom config.
    pub fn with_config(steps: Vec<Arc<dyn Step<Ctx>>>, config: PipelineConfig) -> Self {
        Self { steps, config }
    }

    /// Return a reference to the config.
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Pipeline<Ctx> for DefaultPipeline<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        for step in &self.steps {
            step.execute(ctx).await?;
        }
        Ok(())
    }

    fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for DefaultPipeline<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        Pipeline::execute(self, ctx).await
    }

    fn name(&self) -> &str {
        "default-pipeline"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_with_config() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::with_config(vec![], config.clone());
        assert_eq!(pipeline.config().timeout_per_step, Some(std::time::Duration::from_secs(5)));
    }

    #[test]
    fn test_config() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(10)),
            emit_lifecycle_events: false,
            abort_on_error: true,
        };
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::with_config(vec![], config);
        assert_eq!(pipeline.config().timeout_per_step, Some(std::time::Duration::from_secs(10)));
    }

    #[tokio::test]
    async fn test_empty_pipeline_succeeds() {
        let pipeline = DefaultPipeline::new(vec![]);
        let mut ctx = 0;
        assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
    }

    #[tokio::test]
    async fn test_step_count_returns_correct_count() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    #[tokio::test]
    async fn test_is_empty_returns_correct_status() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert!(pipeline.is_empty());
    }

    #[tokio::test]
    async fn test_default_config() {
        let config = PipelineConfig::default();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }

    #[tokio::test]
    async fn test_pipeline_as_step() {
        let inner: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        let outer: DefaultPipeline<i32> = DefaultPipeline::new(vec![Arc::new(inner)]);

        let mut ctx = 0;
        assert!(Pipeline::execute(&outer, &mut ctx).await.is_ok());
    }
}
