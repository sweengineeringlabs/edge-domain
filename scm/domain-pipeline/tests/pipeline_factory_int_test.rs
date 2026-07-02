#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{
    ContextMutationRequest, PipelineBuilder, PipelineConfig, PipelineSvc, Step,
};
use std::sync::Arc;

#[derive(Clone)]
struct TestPassStep;

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for TestPassStep {
    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Ok(())
    }
}

#[tokio::test]
async fn test_pipeline_factory_create_happy_returns_pipeline() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![Arc::new(TestPassStep)];
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config: PipelineConfig::default(),
        event_bus: None,
    });
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_pipeline_factory_create_with_config_happy_uses_config() {
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![Arc::new(TestPassStep)];
    let config = PipelineConfig::default();
    let pipeline = PipelineSvc::build(PipelineBuilder {
        steps,
        config,
        event_bus: None,
    });
    let mut ctx = 0;
    assert!(pipeline
        .run(ContextMutationRequest { ctx: &mut ctx })
        .await
        .is_ok());
}
