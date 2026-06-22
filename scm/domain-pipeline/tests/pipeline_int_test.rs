//! Integration tests for the [`Pipeline`] trait contract.
//!
//! @covers Pipeline

use edge_domain_pipeline::{create_pipeline, create_pipeline_with_config, Pipeline, Step, PipelineError, DefaultPipeline, AlwaysPassStep, AlwaysFailStep};
use std::sync::Arc;

/// @covers: general
#[tokio::test]
async fn trait_pipeline_executes_all_steps_in_order() {
    struct RecordingStep(&'static str);

    #[async_trait::async_trait]
    impl Step<Vec<&'static str>> for RecordingStep {
        async fn execute(&self, ctx: &mut Vec<&'static str>) -> Result<(), PipelineError> {
            ctx.push(self.0);
            Ok(())
        }

        fn name(&self) -> &str {
            self.0
        }
    }

    let steps: Vec<Arc<dyn Step<Vec<&'static str>>>> = vec![
        Arc::new(RecordingStep("a")),
        Arc::new(RecordingStep("b")),
        Arc::new(RecordingStep("c")),
    ];

    let pipeline = create_pipeline(steps);
    let mut ctx = Vec::new();

    assert!(pipeline.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, vec!["a", "b", "c"]);
}

/// @covers: general
#[tokio::test]
async fn trait_pipeline_step_count_accurate() {
    let pipeline = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
        Arc::new(AlwaysPassStep::new()),
    ]);

    assert_eq!(pipeline.step_count(), 2);
}

/// @covers: general
#[tokio::test]
async fn trait_pipeline_is_empty_works() {
    let empty_pipeline: _ = create_pipeline(vec![]);
    assert!(empty_pipeline.is_empty());

    let nonempty_pipeline: _ = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
    ]);
    assert!(!nonempty_pipeline.is_empty());
}

/// @covers: general
#[tokio::test]
async fn trait_pipeline_halts_on_first_error() {
    let mut step_count = 0;

    struct CountingStep(usize);

    #[async_trait::async_trait]
    impl Step<usize> for CountingStep {
        async fn execute(&self, ctx: &mut usize) -> Result<(), PipelineError> {
            *ctx += 1;
            if self.0 == 2 {
                Err(PipelineError::StepFailed("step 2 failed".to_string()))
            } else {
                Ok(())
            }
        }

        fn name(&self) -> &str {
            "counting"
        }
    }

    let steps: Vec<Arc<dyn Step<usize>>> = vec![
        Arc::new(CountingStep(1)),
        Arc::new(CountingStep(2)),
        Arc::new(CountingStep(3)),
    ];

    let pipeline = create_pipeline(steps);
    let result = pipeline.execute(&mut step_count).await;

    assert!(result.is_err());
    assert_eq!(step_count, 2);
}

/// @covers: general
#[tokio::test]
async fn trait_pipeline_dyn_dispatch_works() {
    let pipeline: Box<dyn Pipeline<i32>> = Box::new(create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
    ]));

    assert_eq!(pipeline.step_count(), 1);
    assert!(!pipeline.is_empty());

    let mut ctx = 0;
    assert!(Pipeline::execute(pipeline.as_ref(), &mut ctx).await.is_ok());
}

/// @covers: general
#[tokio::test]
async fn trait_pipeline_composable_as_step() {
    let inner = create_pipeline(vec![
        Arc::new(AlwaysPassStep::new()),
    ]);

    let outer: Box<dyn Step<i32>> = Box::new(inner);
    let mut ctx = 0;
    assert!(outer.execute(&mut ctx).await.is_ok());
}
