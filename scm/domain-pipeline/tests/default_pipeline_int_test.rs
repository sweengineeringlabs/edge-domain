//! Integration tests for the [`DefaultPipeline`] implementation.
//!
//! @covers DefaultPipeline

use edge_domain_pipeline::{ create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineError, PipelineConfig};
use std::sync::Arc;
use std::time::Duration;

// Test doubles
struct NoopStep;

#[async_trait::async_trait]
impl Step<i32> for NoopStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Ok(())
    }
    fn name(&self) -> &str {
        "noop"
    }
}

struct AlwaysPassStep;

#[async_trait::async_trait]
impl Step<i32> for AlwaysPassStep {
    async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
        Ok(())
    }
    fn name(&self) -> &str {
        "always-pass"
    }
}

impl AlwaysPassStep {
    fn new() -> Self {
        Self
    }
}

struct MutatingStep<F> {
    f: F,
}

impl<F> MutatingStep<F> {
    fn new(f: F) -> Self {
        Self { f }
    }
}

#[async_trait::async_trait]
impl<F: Fn(&mut i32) + Send + Sync> Step<i32> for MutatingStep<F> {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        (self.f)(ctx);
        Ok(())
    }
    fn name(&self) -> &str {
        "mutating"
    }
}

struct PipelineAsStep {
    pipeline: Box<dyn Pipeline<i32>>,
}

impl PipelineAsStep {
    fn new(pipeline: Box<dyn Pipeline<i32>>) -> Self {
        Self { pipeline }
    }
}

#[async_trait::async_trait]
impl Step<i32> for PipelineAsStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        self.pipeline.execute(ctx).await
    }
    fn name(&self) -> &str {
        "pipeline-as-step"
    }
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_executes_sequentially() {
    let mut trace = Vec::new();

    struct TraceStep(usize);

    #[async_trait::async_trait]
    impl Step<Vec<usize>> for TraceStep {
        async fn execute(&self, ctx: &mut Vec<usize>) -> Result<(), PipelineError> {
            ctx.push(self.0);
            Ok(())
        }

        fn name(&self) -> &str {
            "trace"
        }
    }

    let steps: Vec<Arc<dyn Step<Vec<usize>>>> = vec![
        Arc::new(TraceStep(1)),
        Arc::new(TraceStep(2)),
        Arc::new(TraceStep(3)),
    ];

    let pipeline: Box<dyn Pipeline<Vec<usize>>> = create_pipeline(steps);
    assert!(pipeline.execute(&mut trace).await.is_ok());
    assert_eq!(trace, vec![1, 2, 3]);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_with_config_timeout() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(1)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };

    let pipeline: Box<dyn Pipeline<i32>> = create_pipeline_with_config(
        vec![Arc::new(NoopStep)],
        config.clone(),
    );

    assert_eq!(pipeline.config().timeout_per_step, Some(Duration::from_secs(1)));
    assert_eq!(pipeline.config().abort_on_error, true);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_abort_on_error_true() {
    let mut exec_count = 0;

    struct CountingFailStep;

    #[async_trait::async_trait]
    impl Step<usize> for CountingFailStep {
        async fn execute(&self, ctx: &mut usize) -> Result<(), PipelineError> {
            *ctx += 1;
            if *ctx == 2 {
                Err(PipelineError::StepFailed("fail at 2".to_string()))
            } else {
                Ok(())
            }
        }

        fn name(&self) -> &str {
            "counter"
        }
    }

    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };

    let steps: Vec<Arc<dyn Step<usize>>> = vec![
        Arc::new(CountingFailStep),
        Arc::new(CountingFailStep),
        Arc::new(CountingFailStep),
    ];

    let pipeline: Box<dyn Pipeline<usize>> = create_pipeline_with_config(steps, config);
    let result = pipeline.execute(&mut exec_count).await;

    assert!(result.is_err());
    assert_eq!(exec_count, 2);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_config_with_lifecycle_events() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: true,
        abort_on_error: true,
    };

    let pipeline: Box<dyn Pipeline<i32>> = create_pipeline_with_config(vec![], config.clone());
    assert!(pipeline.config().emit_lifecycle_events);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_as_step_nesting() {
    let inner = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(AlwaysPassStep::new()),
    ]);

    let inner_as_step: Arc<dyn Step<i32>> = Arc::new(PipelineAsStep::new(inner));
    let outer: Box<dyn Pipeline<i32>> = create_pipeline(vec![inner_as_step]);
    let mut ctx = 0;
    assert!(outer.execute(&mut ctx).await.is_ok());
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_with_mixed_steps() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::new(|ctx: &mut i32| *ctx += 5)),
        Arc::new(AlwaysPassStep::new()),
    ];

    let pipeline = create_pipeline(steps);
    let mut ctx = 10;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_short_circuits_on_fail() {
    let mut executed = Vec::new();

    struct RecordingFailStep(&'static str);

    #[async_trait::async_trait]
    impl Step<Vec<&'static str>> for RecordingFailStep {
        async fn execute(&self, ctx: &mut Vec<&'static str>) -> Result<(), PipelineError> {
            ctx.push(self.0);
            if self.0 == "b" {
                Err(PipelineError::StepFailed("b failed".to_string()))
            } else {
                Ok(())
            }
        }

        fn name(&self) -> &str {
            self.0
        }
    }

    let steps: Vec<Arc<dyn Step<Vec<&'static str>>>> = vec![
        Arc::new(RecordingFailStep("a")),
        Arc::new(RecordingFailStep("b")),
        Arc::new(RecordingFailStep("c")),
    ];

    let pipeline = create_pipeline(steps);
    let result = pipeline.execute(&mut executed).await;

    assert!(result.is_err());
    assert_eq!(executed, vec!["a", "b"]);
}
