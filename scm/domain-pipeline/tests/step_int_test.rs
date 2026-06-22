//! Integration tests for the [`Step`] trait contract.
//!
//! @covers Step

use edge_domain_pipeline::{Step, PipelineError};

struct CountingStep {
    name: String,
}

#[async_trait::async_trait]
impl Step<i32> for CountingStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
        *ctx += 1;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// @covers: general
#[tokio::test]
async fn trait_step_executes_and_mutates_context() {
    let step = CountingStep {
        name: "increment".to_string(),
    };
    let mut ctx = 5;
    assert!(step.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 6);
}

/// @covers: general
#[tokio::test]
async fn trait_step_name_is_accessible() {
    let step = CountingStep {
        name: "my-step".to_string(),
    };
    assert_eq!(step.name(), "my-step");
}

/// @covers: general
#[tokio::test]
async fn trait_step_error_halts_mutation() {
    struct FailingStep;

    #[async_trait::async_trait]
    impl Step<String> for FailingStep {
        async fn execute(&self, _ctx: &mut String) -> Result<(), PipelineError> {
            Err(PipelineError::StepFailed("forced failure".to_string()))
        }

        fn name(&self) -> &str {
            "failing-step"
        }
    }

    let step = FailingStep;
    let mut ctx = "initial".to_string();
    let result = step.execute(&mut ctx).await;
    assert!(result.is_err());
    assert_eq!(ctx, "initial");
}

/// @covers: general
#[tokio::test]
async fn trait_step_dyn_dispatch_works() {
    let step1: Box<dyn Step<i32>> = Box::new(CountingStep {
        name: "step1".to_string(),
    });
    let step2: Box<dyn Step<i32>> = Box::new(CountingStep {
        name: "step2".to_string(),
    });

    let mut ctx = 0;
    assert!(step1.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 1);

    assert!(step2.execute(&mut ctx).await.is_ok());
    assert_eq!(ctx, 2);
}
