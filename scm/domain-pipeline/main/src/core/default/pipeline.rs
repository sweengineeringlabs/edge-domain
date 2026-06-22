//! [`DefaultPipeline<Ctx>`] — orchestrates sequential step execution.

use std::sync::Arc;

use crate::api::{Pipeline, PipelineConfig, PipelineError, Step};

/// Executes a sequence of steps in order, passing context through each.
#[derive(Clone)]
pub(crate) struct DefaultPipeline<Ctx> {
    steps: Vec<Arc<dyn Step<Ctx>>>,
    config: PipelineConfig,
}

impl<Ctx: Send> DefaultPipeline<Ctx> {
    /// Create a new pipeline with given steps and default config.
    pub(crate) fn new(steps: Vec<Arc<dyn Step<Ctx>>>) -> Self {
        Self {
            steps,
            config: PipelineConfig::default(),
        }
    }

    /// Create a new pipeline with custom config.
    pub(crate) fn with_config(steps: Vec<Arc<dyn Step<Ctx>>>, config: PipelineConfig) -> Self {
        Self { steps, config }
    }

    /// Return a reference to the config.
    pub(crate) fn config(&self) -> &PipelineConfig {
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
        "default-pipeline"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CountingStep {
        call_count: Arc<std::sync::Mutex<usize>>,
    }

    #[async_trait::async_trait]
    impl Step<i32> for CountingStep {
        async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            *ctx += 1;
            Ok(())
        }

        fn name(&self) -> &str {
            "counting-step"
        }
    }

    struct FailingStep {
        msg: String,
    }

    #[async_trait::async_trait]
    impl Step<i32> for FailingStep {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
            Err(PipelineError::StepFailed(self.msg.clone()))
        }

        fn name(&self) -> &str {
            "failing-step"
        }
    }

    // Tests for execute()
    /// @covers: execute
    #[tokio::test]
    async fn test_execute_happy_empty_pipeline() {
        let pipeline = DefaultPipeline::new(vec![]);
        let mut ctx = 0;
        assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
        assert_eq!(ctx, 0);
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_happy_single_step() {
        let step = Arc::new(CountingStep {
            call_count: Arc::new(std::sync::Mutex::new(0)),
        });
        let pipeline = DefaultPipeline::new(vec![step.clone()]);
        let mut ctx = 0;
        assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
        assert_eq!(ctx, 1);
        assert_eq!(*step.call_count.lock().unwrap(), 1);
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_happy_multiple_steps() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let step1 = Arc::new(CountingStep { call_count: counter.clone() });
        let step2 = Arc::new(CountingStep { call_count: counter.clone() });
        let step3 = Arc::new(CountingStep { call_count: counter.clone() });
        let pipeline = DefaultPipeline::new(vec![step1, step2, step3]);
        let mut ctx = 0;
        assert!(Pipeline::execute(&pipeline, &mut ctx).await.is_ok());
        assert_eq!(ctx, 3);
        assert_eq!(*counter.lock().unwrap(), 3);
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_error_halts_pipeline() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let step1 = Arc::new(CountingStep { call_count: counter.clone() });
        let step2 = Arc::new(FailingStep { msg: "test error".to_string() });
        let step3 = Arc::new(CountingStep { call_count: counter.clone() });
        let pipeline = DefaultPipeline::new(vec![step1, step2, step3]);
        let mut ctx = 0;
        let result = Pipeline::execute(&pipeline, &mut ctx).await;
        assert!(result.is_err());
        assert_eq!(ctx, 1); // Only first step executed
        assert_eq!(*counter.lock().unwrap(), 1); // Only first step counted
    }

    // Tests for step_count()
    /// @covers: step_count
    #[test]
    fn test_step_count_happy_empty() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    /// @covers: step_count
    #[test]
    fn test_step_count_happy_single() {
        let step = Arc::new(CountingStep {
            call_count: Arc::new(std::sync::Mutex::new(0)),
        });
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![step]);
        assert_eq!(pipeline.step_count(), 1);
    }

    /// @covers: step_count
    #[test]
    fn test_step_count_happy_multiple() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let steps: Vec<Arc<dyn Step<i32>>> = vec![
            Arc::new(CountingStep { call_count: counter.clone() }),
            Arc::new(CountingStep { call_count: counter.clone() }),
            Arc::new(CountingStep { call_count: counter.clone() }),
            Arc::new(CountingStep { call_count: counter.clone() }),
            Arc::new(CountingStep { call_count: counter.clone() }),
        ];
        let pipeline = DefaultPipeline::new(steps);
        assert_eq!(pipeline.step_count(), 5);
    }

    // Tests for is_empty()
    /// @covers: is_empty
    #[test]
    fn test_is_empty_happy_true() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert!(pipeline.is_empty());
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_happy_false() {
        let step = Arc::new(CountingStep {
            call_count: Arc::new(std::sync::Mutex::new(0)),
        });
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![step]);
        assert!(!pipeline.is_empty());
    }

    // Tests for config()
    /// @covers: config
    #[test]
    fn test_config_happy_default() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        let config = pipeline.config();
        assert!(config.timeout_per_step.is_none());
        assert!(!config.emit_lifecycle_events);
        assert!(config.abort_on_error);
    }

    /// @covers: config
    #[test]
    fn test_config_happy_custom() {
        let custom_config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(30)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::with_config(vec![], custom_config);
        let config = pipeline.config();
        assert_eq!(config.timeout_per_step, Some(std::time::Duration::from_secs(30)));
        assert!(config.emit_lifecycle_events);
        assert!(!config.abort_on_error);
    }

    // Tests for name() (from Step trait impl)
    /// @covers: Step::name
    #[test]
    fn test_name_happy_returns_default_pipeline() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        let step_ref: &dyn Step<i32> = &pipeline;
        assert_eq!(step_ref.name(), "default-pipeline");
    }

    // Composition and integration tests
    #[tokio::test]
    async fn test_pipeline_as_step_composition() {
        let inner: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        let outer: DefaultPipeline<i32> = DefaultPipeline::new(vec![Arc::new(inner)]);
        let mut ctx = 0;
        assert!(Pipeline::execute(&outer, &mut ctx).await.is_ok());
    }

    #[test]
    fn test_new_constructor() {
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::new(vec![]);
        assert_eq!(pipeline.step_count(), 0);
    }

    #[test]
    fn test_with_config_constructor() {
        let config = PipelineConfig {
            timeout_per_step: Some(std::time::Duration::from_secs(5)),
            emit_lifecycle_events: true,
            abort_on_error: false,
        };
        let pipeline: DefaultPipeline<i32> = DefaultPipeline::with_config(vec![], config.clone());
        assert_eq!(pipeline.config().timeout_per_step, Some(std::time::Duration::from_secs(5)));
    }
}
