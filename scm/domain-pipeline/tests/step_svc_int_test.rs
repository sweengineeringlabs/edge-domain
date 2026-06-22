//! Integration tests for step service facade.

use edge_domain_pipeline::{Step, PipelineError, STEP_SVC};
use std::sync::Arc;

struct TestStep(i32);

#[async_trait::async_trait]
impl Step<i32> for TestStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += self.0;
        Ok(())
    }

    fn name(&self) -> &str {
        "test"
    }
}

// Test STEP_SVC constant
/// @covers: general
#[test]
fn test_step_svc_constant() {
    assert_eq!(STEP_SVC, "step");
}

// Test Step trait usage through factory
/// @covers: general
#[tokio::test]
async fn test_step_svc_step_trait_happy_execute() {
    let step: Arc<dyn Step<i32>> = Arc::new(TestStep(5));
    let mut ctx = 10;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 15);
}

/// @covers: general
#[test]
fn test_step_svc_step_trait_happy_name() {
    let step: Box<dyn Step<i32>> = Box::new(TestStep(0));
    assert_eq!(step.name(), "test");
}

/// @covers: general
#[tokio::test]
async fn test_step_svc_step_trait_edge_different_values() {
    let step1: Arc<dyn Step<i32>> = Arc::new(TestStep(10));
    let step2: Arc<dyn Step<i32>> = Arc::new(TestStep(-5));

    let mut ctx1 = 0;
    let mut ctx2 = 0;

    assert!(step1.execute(&mut ctx1).await.is_ok());
    assert_eq!(ctx1, 10);

    assert!(step2.execute(&mut ctx2).await.is_ok());
    assert_eq!(ctx2, -5);
}
