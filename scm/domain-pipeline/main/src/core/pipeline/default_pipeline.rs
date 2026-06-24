//! [`DefaultPipeline<Ctx>`] — orchestrates sequential step execution.

use std::sync::Arc;

use tokio::time;

use crate::api::{Pipeline, PipelineConfig, PipelineError, Step};

/// Executes a sequence of steps in order, passing context through each.
#[derive(Clone)]
pub(crate) struct DefaultPipeline<Ctx> {
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
    step_name: &'static str,
}

impl<Ctx: Send> DefaultPipeline<Ctx> {
    /// Create a new pipeline with given steps and default config.
    pub(crate) fn new(steps: Vec<Arc<dyn Step<Ctx>>>) -> Self {
        Self {
            steps,
            config: PipelineConfig::default(),
            step_name: "default-pipeline",
        }
    }

    /// Create a new pipeline with custom config.
    pub(crate) fn with_config(steps: Vec<Arc<dyn Step<Ctx>>>, config: PipelineConfig) -> Self {
        Self {
            steps,
            config,
            step_name: "default-pipeline",
        }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Pipeline<Ctx> for DefaultPipeline<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        for step in &self.steps {
            let result = match self.config.timeout_per_step {
                Some(dur) => match time::timeout(dur, step.execute(ctx)).await {
                    Ok(r) => r,
                    Err(_elapsed) => Err(PipelineError::StepTimeout),
                },
                None => step.execute(ctx).await,
            };
            if let Err(e) = result {
                if self.config.abort_on_error {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn step_count(&self) -> usize {
        self.steps.len()
    }

    fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for DefaultPipeline<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        Pipeline::execute(self, ctx).await
    }

    fn name(&self) -> &str {
        self.step_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_happy_creates_empty() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    /// @covers: with_config
    #[test]
    fn test_with_config_happy_sets_timeout() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::with_config(vec![], config.clone());
        assert_eq!(
            pipeline.config().timeout_per_step,
            Some(std::time::Duration::from_secs(5))
        );
    }

    /// @covers: config
    #[test]
    fn test_config_happy_returns_defaults() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        let config = pipeline.config();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }
}
