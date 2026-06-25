use edge_domain_pipeline::{PipelineBuilder, PipelineConfig, PipelineError, PipelineSvc, Step};
use std::sync::Arc;

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
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config: PipelineConfig::default() });
    let mut ctx = 0;
    assert!(pipeline.run(&mut ctx).await.is_ok());
}

#[tokio::test]
async fn test_pipeline_factory_create_with_config_happy_uses_config() {
    let steps: Vec<Arc<dyn Step<i32>>> = vec![Arc::new(TestPassStep)];
    let config = PipelineConfig::default();
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config });
    let mut ctx = 0;
    assert!(pipeline.run(&mut ctx).await.is_ok());
}
