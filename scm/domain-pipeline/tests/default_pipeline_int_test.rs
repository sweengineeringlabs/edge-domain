//! Integration tests for the [`DefaultPipeline`] implementation.
//!
//! @covers DefaultPipeline
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineError, PipelineSvc, Step, StepNameRequest, StepNameResponse,
};
use std::sync::Arc;
use std::time::Duration;

struct NoopStep;

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for NoopStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        let _ = req;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "noop".to_string(),
        })
    }
}

struct AlwaysPassStep;

impl AlwaysPassStep {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl<E: Send + 'static> Step<i32, E> for AlwaysPassStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        let _ = req;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "always-pass".to_string(),
        })
    }
}

struct MutatingStep<F, E> {
    f: F,
    _phantom: std::marker::PhantomData<fn(E)>,
}

impl<F, E> MutatingStep<F, E> {
    fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<F: Fn(&mut i32) + Send + Sync, E: Send + 'static> Step<i32, E> for MutatingStep<F, E> {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), E> {
        (self.f)(req.ctx);
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "mutating".to_string(),
        })
    }
}

// PipelineAsStep wraps an inner pipeline, mapping its PipelineError<String> to String
struct PipelineAsStep {
    pipeline: Box<dyn Pipeline<i32, String>>,
}

impl PipelineAsStep {
    fn new(pipeline: Box<dyn Pipeline<i32, String>>) -> Self {
        Self { pipeline }
    }
}

#[async_trait::async_trait]
impl Step<i32, String> for PipelineAsStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        self.pipeline
            .run(ContextMutationRequest { ctx: req.ctx })
            .await
            .map_err(|e| e.to_string())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "pipeline-as-step".to_string(),
        })
    }
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_executes_sequentially() {
    struct TraceStep(usize);

    #[async_trait::async_trait]
    impl<E: Send + 'static> Step<Vec<usize>, E> for TraceStep {
        async fn execute(&self, req: ContextMutationRequest<'_, Vec<usize>>) -> Result<(), E> {
            req.ctx.push(self.0);
            Ok(())
        }
        fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
            Ok(StepNameResponse {
                name: "trace".to_string(),
            })
        }
    }

    let mut trace = Vec::new();
    let steps: Vec<Arc<dyn Step<Vec<usize>, String>>> = vec![
        Arc::new(TraceStep(1)),
        Arc::new(TraceStep(2)),
        Arc::new(TraceStep(3)),
    ];
    let pipeline: Box<dyn Pipeline<Vec<usize>, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut trace })
        .await
        .is_ok());
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
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![Arc::new(NoopStep)];
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config,
        event_bus: None,
    });
    let resolved_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(
        resolved_config.timeout_per_step,
        Some(Duration::from_secs(1))
    );
    assert!(resolved_config.abort_on_error);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_abort_on_error_true() {
    struct CountingFailStep;

    #[async_trait::async_trait]
    impl Step<usize, String> for CountingFailStep {
        async fn execute(&self, req: ContextMutationRequest<'_, usize>) -> Result<(), String> {
            *req.ctx += 1;
            if *req.ctx == 2 {
                Err("fail at 2".to_string())
            } else {
                Ok(())
            }
        }
        fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
            Ok(StepNameResponse {
                name: "counter".to_string(),
            })
        }
    }

    let mut exec_count = 0usize;
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let steps: Vec<Arc<dyn Step<usize, String>>> = vec![
        Arc::new(CountingFailStep),
        Arc::new(CountingFailStep),
        Arc::new(CountingFailStep),
    ];
    let pipeline: Box<dyn Pipeline<usize, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config,
        event_bus: None,
    });
    let result = pipeline
        .run(ContextMutationRequest {
            ctx: &mut exec_count,
        })
        .await;
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
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .emit_lifecycle_events
    );
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_as_step_nesting() {
    let inner: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(
        PipelineBuilder::new()
            .with(AlwaysPassStep::new())
            .with(AlwaysPassStep::new()),
    );
    let inner_as_step: Arc<dyn Step<i32, String>> = Arc::new(PipelineAsStep::new(inner));
    let outer: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(inner_as_step));
    let mut ctx = 0;
    assert!(outer
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_with_mixed_steps() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::<_, String>::new(|ctx: &mut i32| *ctx += 5)),
        Arc::new(AlwaysPassStep::new()),
    ];
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 10;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: general
#[tokio::test]
async fn struct_default_pipeline_short_circuits_on_fail() {
    struct RecordingFailStep(&'static str);

    #[async_trait::async_trait]
    impl Step<Vec<&'static str>, String> for RecordingFailStep {
        async fn execute(
            &self,
            req: ContextMutationRequest<'_, Vec<&'static str>>,
        ) -> Result<(), String> {
            req.ctx.push(self.0);
            if self.0 == "b" {
                Err("b failed".to_string())
            } else {
                Ok(())
            }
        }
        fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
            Ok(StepNameResponse {
                name: self.0.to_string(),
            })
        }
    }

    let mut executed: Vec<&'static str> = Vec::new();
    let steps: Vec<Arc<dyn Step<Vec<&'static str>, String>>> = vec![
        Arc::new(RecordingFailStep("a")),
        Arc::new(RecordingFailStep("b")),
        Arc::new(RecordingFailStep("c")),
    ];
    let pipeline: Box<dyn Pipeline<Vec<&'static str>, String>> =
        PipelineSvc::build(PipelineBuilder {
            steps,
            config: PipelineConfig::default(),
            event_bus: None,
        });
    let result = pipeline
        .run(ContextMutationRequest { ctx: &mut executed })
        .await;
    assert!(result.is_err());
    assert_eq!(executed, vec!["a", "b"]);
}
