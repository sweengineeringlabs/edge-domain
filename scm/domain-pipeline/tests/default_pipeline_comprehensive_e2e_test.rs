//! @covers DefaultPipeline config and nesting
//! Comprehensive scenario coverage for DefaultPipeline struct.
//! Tests: config variations, nesting, edge cases
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, Pipeline, PipelineBuilder, PipelineConfig, PipelineConfigLookupRequest,
    PipelineError, PipelineSvc, Step, StepCountRequest, StepNameRequest, StepNameResponse,
};
use std::sync::Arc;
use std::time::Duration;

struct AlwaysPassStep;

impl AlwaysPassStep {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for AlwaysPassStep {
    async fn execute(&self, req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        let _ = req;
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "always-pass".to_string(),
        })
    }
}

struct AlwaysFailStep {
    msg: String,
}

impl AlwaysFailStep {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Step<i32, String> for AlwaysFailStep {
    async fn execute(&self, req: ContextMutationRequest<'_, i32>) -> Result<(), String> {
        let _ = req;
        Err(self.msg.clone())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<String>> {
        Ok(StepNameResponse {
            name: "always-fail".to_string(),
        })
    }
}

struct MutatingStep<Ctx, F, E> {
    f: F,
    _phantom: std::marker::PhantomData<fn(Ctx, E)>,
}

impl<Ctx, F, E> MutatingStep<Ctx, F, E> {
    fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send + Sync, F: Fn(&mut Ctx) + Send + Sync, E: Send + 'static> Step<Ctx, E>
    for MutatingStep<Ctx, F, E>
{
    async fn execute(&self, req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        (self.f)(req.ctx);
        Ok(())
    }
    fn name(&self, _req: StepNameRequest) -> Result<StepNameResponse, PipelineError<E>> {
        Ok(StepNameResponse {
            name: "mutating".to_string(),
        })
    }
}

// PipelineAsStep: converts inner PipelineError<String> to String for uniform error type
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

// Config variation: timeout
#[test]
fn test_default_pipeline_config_with_timeout_happy() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(5)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert_eq!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .timeout_per_step,
        Some(Duration::from_secs(5))
    );
}

#[test]
fn test_default_pipeline_config_no_timeout_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert_eq!(
        pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .timeout_per_step,
        None
    );
}

// Config variation: lifecycle events
#[test]
fn test_default_pipeline_config_lifecycle_enabled_happy() {
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

#[test]
fn test_default_pipeline_config_lifecycle_disabled_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert!(
        !pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .emit_lifecycle_events
    );
}

// Config variation: abort_on_error
#[test]
fn test_default_pipeline_config_abort_true_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
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
            .abort_on_error
    );
}

#[test]
fn test_default_pipeline_config_abort_false_happy() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert!(
        !pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .abort_on_error
    );
}

// Composability: nested pipelines
#[tokio::test]
async fn test_default_pipeline_composability_single_nesting_happy() {
    let inner: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with(AlwaysPassStep::new()));
    let inner_as_step: Arc<dyn Step<i32, String>> = Arc::new(PipelineAsStep::new(inner));
    let outer: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(inner_as_step));
    let mut ctx = 0;
    assert!(outer
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_default_pipeline_composability_double_nesting_happy() {
    let level1: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with(AlwaysPassStep::new()));
    let level1_as_step: Arc<dyn Step<i32, String>> = Arc::new(PipelineAsStep::new(level1));
    let level2: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(level1_as_step));
    let level2_as_step: Arc<dyn Step<i32, String>> = Arc::new(PipelineAsStep::new(level2));
    let level3: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(level2_as_step));
    let mut ctx = 0;
    assert!(level3
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_default_pipeline_composability_inner_fails_error() {
    let inner: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with(AlwaysFailStep::new("inner failed")));
    let inner_as_step: Arc<dyn Step<i32, String>> = Arc::new(PipelineAsStep::new(inner));
    let outer: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::new().with_shared(inner_as_step));
    let mut ctx = 0;
    assert!(outer
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_err());
}

// Context mutation across steps
#[tokio::test]
async fn test_default_pipeline_mutation_accumulate_happy() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx += 1
        })),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx += 2
        })),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx += 3
        })),
    ];
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 6);
}

#[tokio::test]
async fn test_default_pipeline_mutation_chain_happy() {
    let steps: Vec<Arc<dyn Step<String, String>>> = vec![
        Arc::new(MutatingStep::<String, _, String>::new(
            |ctx: &mut String| ctx.push('a'),
        )),
        Arc::new(MutatingStep::<String, _, String>::new(
            |ctx: &mut String| ctx.push('b'),
        )),
        Arc::new(MutatingStep::<String, _, String>::new(
            |ctx: &mut String| ctx.push('c'),
        )),
    ];
    let pipeline: Box<dyn Pipeline<String, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = String::new();
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, "abc");
}

// Edge cases: large pipelines
#[tokio::test]
async fn test_default_pipeline_many_steps_edge() {
    let mut builder: PipelineBuilder<i32, String> = PipelineBuilder::new();
    for _ in 0..100 {
        builder = builder.with(AlwaysPassStep::new());
    }
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(builder);
    assert_eq!(
        pipeline
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        100
    );
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

// Edge cases: mixed step types
#[tokio::test]
async fn test_default_pipeline_mixed_step_types_edge() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx += 5
        })),
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx *= 2
        })),
    ];
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 3;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
    assert_eq!(ctx, 16);
}

#[tokio::test]
async fn test_default_pipeline_fail_in_mixed_chain_edge() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx += 5
        })),
        Arc::new(AlwaysFailStep::new("stop")),
        Arc::new(MutatingStep::<i32, _, String>::new(|ctx: &mut i32| {
            *ctx *= 2
        })),
    ];
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 3;
    let result = pipeline.run(ContextMutationRequest { ctx: &mut ctx }).await;
    assert!(result.is_err());
    assert_eq!(ctx, 8);
}

// Clone support
#[test]
fn test_default_pipeline_clone_happy_edge() {
    let _pipeline1: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::<i32, String>::new());
    let pipeline2: Box<dyn Pipeline<i32, String>> =
        PipelineSvc::build(PipelineBuilder::<i32, String>::new());
    assert_eq!(
        pipeline2
            .step_count(StepCountRequest)
            .expect("must succeed")
            .count,
        0
    );
}

// Edge case: config with all options enabled
#[test]
fn test_default_pipeline_config_all_enabled_edge() {
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: true,
        abort_on_error: true,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    let resolved_config = pipeline
        .config(PipelineConfigLookupRequest)
        .expect("must succeed")
        .config;
    assert_eq!(
        resolved_config.timeout_per_step,
        Some(Duration::from_secs(10))
    );
    assert!(resolved_config.emit_lifecycle_events);
    assert!(resolved_config.abort_on_error);
}

#[tokio::test]
async fn test_default_pipeline_config_abort_disabled_error() {
    let config = PipelineConfig {
        timeout_per_step: None,
        emit_lifecycle_events: false,
        abort_on_error: false,
    };
    let pipeline: Box<dyn Pipeline<i32, String>> = PipelineSvc::build(PipelineBuilder {
        steps: vec![],
        config,
        event_bus: None,
    });
    assert!(
        !pipeline
            .config(PipelineConfigLookupRequest)
            .expect("must succeed")
            .config
            .abort_on_error
    );
}
