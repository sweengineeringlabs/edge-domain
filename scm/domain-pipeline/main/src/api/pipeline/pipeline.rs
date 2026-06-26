//! [`Pipeline<Ctx>`] — orchestrates a sequence of steps.

use edge_domain_service::Service;

use super::super::error::PipelineError;
use super::super::types::PipelineBuilder;

/// Orchestrates a sequence of [`Step`] operations.
///
/// `Pipeline<Ctx>` extends [`Service`] with `Request = Ctx` and `Response = Ctx`,
/// making every pipeline a first-class domain service. The dispatcher bridge
/// ([`edge_domain_handler::IntoHandler`]) fires automatically on any `Pipeline<Ctx>`
/// implementor — no wrapper required.
///
/// The pipeline executes steps in order, passing a mutable context through each step.
/// Each step enriches or validates the context. If any step fails, the pipeline
/// halts and returns the error.
///
/// # Invariant
///
/// Steps execute sequentially. The pipeline is not parallel.
#[async_trait::async_trait]
pub trait Pipeline<Ctx: Send + 'static>: Service<Request = Ctx, Response = Ctx> {
    /// Run all steps in order, passing a mutable context through each.
    ///
    /// Steps execute sequentially; the context is mutated in-place. On the
    /// first step error, execution stops and the error is returned (unless
    /// `abort_on_error` is `false` in the pipeline config).
    ///
    /// Distinct from [`Service::execute`] (which takes ownership for dispatcher
    /// bridge use): `run` is the step-by-step orchestration entry point.
    ///
    /// # Errors
    ///
    /// Returns the first [`PipelineError`] encountered. The context may be
    /// partially mutated from earlier steps.
    async fn run(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;

    /// Return the number of steps in this pipeline.
    fn step_count(&self) -> usize;

    /// Return true if the pipeline has no steps.
    fn is_empty(&self) -> bool {
        self.step_count() == 0
    }

    /// Get the pipeline configuration.
    fn config(&self) -> &super::super::PipelineConfig;

    /// Create a new fluent builder for assembling a pipeline.
    fn new_builder() -> PipelineBuilder<Ctx>
    where
        Self: Sized,
    {
        PipelineBuilder {
            steps: Vec::new(),
            config: super::super::PipelineConfig::default(),
            event_bus: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::PipelineConfig;
    use edge_domain_service::ServiceError;
    use futures::future::BoxFuture;

    struct MockPipeline {
        empty: bool,
        config: PipelineConfig,
    }

    impl Service for MockPipeline {
        type Request = i32;
        type Response = i32;

        fn name(&self) -> &str {
            "mock.pipeline"
        }

        fn execute(&self, ctx: i32) -> BoxFuture<'_, Result<i32, ServiceError>> {
            Box::pin(async move { Ok(ctx) })
        }
    }

    #[async_trait::async_trait]
    impl Pipeline<i32> for MockPipeline {
        async fn run(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
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
    fn test_pipeline_is_empty_error_consistency() {
        let pipeline = MockPipeline { empty: true, config: PipelineConfig::default() };
        assert!(pipeline.is_empty());
        assert!(pipeline.is_empty());
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
    fn test_pipeline_step_count_error_consistency() {
        let pipeline = MockPipeline { empty: false, config: PipelineConfig::default() };
        let count1 = pipeline.step_count();
        let count2 = pipeline.step_count();
        assert_eq!(count1, count2);
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

    #[test]
    fn test_pipeline_config_error_multiple_calls_consistent() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: false,
            abort_on_error: true,
        };
        let pipeline = MockPipeline { empty: false, config };
        let ref1 = pipeline.config();
        let ref2 = pipeline.config();
        assert_eq!(ref1.timeout_per_step, ref2.timeout_per_step);
    }
}
