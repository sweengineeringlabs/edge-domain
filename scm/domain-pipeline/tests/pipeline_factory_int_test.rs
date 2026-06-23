use edge_domain_pipeline::{Pipeline, Step, PipelineError, PipelineConfig, create_pipeline, create_pipeline_with_config};
use std::sync::Arc;

/// A test step that always passes.
#[derive(Clone)]
struct TestPassStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for TestPassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "test-pass"
    }
}

#[tokio::test]
async fn test_pipeline_factory_create_happy_returns_pipeline() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep)];
    let pipeline = create_pipeline(steps);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_pipeline_factory_create_with_config_happy_uses_config() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep)];
    let config = PipelineConfig::default();
    let pipeline = create_pipeline_with_config(steps, config);
    let mut ctx = 0;
    assert!(pipeline.execute(&mut ctx).await.is_ok());
}
