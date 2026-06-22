//! [`Pipeline<Ctx>`] — orchestrates a sequence of steps.

use super::super::error::PipelineError;
use super::step::Step;

/// Orchestrates a sequence of [`Step`] operations.
///
/// The pipeline executes steps in order, passing a mutable context through each step.
/// Each step enriches or validates the context. If any step fails, the pipeline
/// halts and returns the error.
///
/// # Invariant
///
/// Steps execute sequentially. The pipeline is not parallel.
#[async_trait::async_trait]
pub trait Pipeline<Ctx>: Send + Sync {
    /// Execute all steps in order.
    ///
    /// Steps are run sequentially. Context is mutated in-place by each step.
    /// If any step returns an error, execution stops and that error is returned.
    ///
    /// # Errors
    ///
    /// Returns the first [`PipelineError`] encountered. The context may be
    /// partially mutated from earlier steps.
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;

    /// Return the number of steps in this pipeline.
    fn step_count(&self) -> usize;

    /// Return true if the pipeline has no steps.
    fn is_empty(&self) -> bool {
        self.step_count() == 0
    }

    /// Get the pipeline configuration.
    fn config(&self) -> &super::super::PipelineConfig;

    /// Return the name of this pipeline.
    fn name(&self) -> &str {
        "pipeline"
    }
}

/// Blanket impl: any Pipeline can be used as a Step, enabling composition.
#[async_trait::async_trait]
impl<Ctx: Send + 'static> Step<Ctx> for dyn Pipeline<Ctx> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        Pipeline::execute(self, ctx).await
    }

    fn name(&self) -> &str {
        "pipeline-step"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::PipelineConfig;

    struct MockPipeline {
        empty: bool,
        config: PipelineConfig,
    }

    #[async_trait::async_trait]
    impl Pipeline<i32> for MockPipeline {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
            Ok(())
        }

        fn step_count(&self) -> usize {
            if self.empty { 0 } else { 1 }
        }

        fn config(&self) -> &PipelineConfig {
            &self.config
        }
    }

    #[test]
    fn test_pipeline_name_happy_default() {
        let pipeline = MockPipeline { empty: false, config: PipelineConfig::default() };
        assert_eq!(pipeline.name(), "pipeline");
    }

    #[test]
    fn test_pipeline_is_empty_happy_true() {
        let pipeline = MockPipeline { empty: true, config: PipelineConfig::default() };
        assert!(pipeline.is_empty());
    }

    #[test]
    fn test_pipeline_is_empty_happy_false() {
        let pipeline = MockPipeline { empty: false, config: PipelineConfig::default() };
        assert!(!pipeline.is_empty());
    }

    #[test]
    fn test_pipeline_step_count_happy_returns_count() {
        let pipeline = MockPipeline { empty: false, config: PipelineConfig::default() };
        assert_eq!(pipeline.step_count(), 1);
    }

    #[test]
    fn test_pipeline_step_count_edge_empty_zero() {
        let pipeline = MockPipeline { empty: true, config: PipelineConfig::default() };
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_pipeline_config_happy_returns_reference() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(10)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline = MockPipeline { empty: false, config: config.clone() };
        assert_eq!(pipeline.config().timeout_per_step, Some(std::time::Duration::from_secs(10)));
        assert!(pipeline.config().emit_lifecycle_events);
        assert!(!pipeline.config().abort_on_error);
    }

    #[test]
    fn test_pipeline_config_edge_defaults() {
        let pipeline = MockPipeline { empty: false, config: PipelineConfig::default() };
        assert!(pipeline.config().timeout_per_step.is_none());
        assert!(!pipeline.config().emit_lifecycle_events);
        assert!(pipeline.config().abort_on_error);
    }
}
