//! [`Step<Ctx>`] — single composable operation in a pipeline.

use super::super::error::PipelineError;

/// A single composable step in a [`Pipeline`](super::pipeline::Pipeline).
///
/// Steps execute sequentially, each receiving a mutable reference to shared context.
/// A step either succeeds (mutating context), fails (aborting the pipeline), or times out.
///
/// # Invariant
///
/// A step must be idempotent over the context — calling it twice with the same context
/// must produce the same result or error.
///
/// # Error Semantics
///
/// - `Ok(())` — Step succeeded; context mutations applied; continue to next step
/// - `Err(PipelineError::StepFailed(_))` — Step failed; abort pipeline
/// - `Err(PipelineError::StepTimeout)` — Step exceeded timeout; abort or skip based on config
/// - `Err(PipelineError::ConfigError(_))` — Configuration error; abort pipeline
#[async_trait::async_trait]
pub trait Step<Ctx>: Send + Sync {
    /// Execute this step, mutating the context in-place.
    ///
    /// # Errors
    ///
    /// Returns [`PipelineError`] if the step fails. The context may be partially
    /// mutated before the error; callers must not assume rollback.
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError>;

    /// Human-readable name for this step (logging, debugging, observability).
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SuccessStep;

    #[async_trait::async_trait]
    impl Step<i32> for SuccessStep {
        async fn execute(&self, ctx: &mut i32) -> Result<(), PipelineError> {
            *ctx += 1;
            Ok(())
        }

        fn name(&self) -> &str {
            "success-step"
        }
    }

    struct FailureStep;

    #[async_trait::async_trait]
    impl Step<i32> for FailureStep {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), PipelineError> {
            Err(PipelineError::StepFailed("test failure".to_string()))
        }

        fn name(&self) -> &str {
            "failure-step"
        }
    }

    #[tokio::test]
    async fn test_execute_happy_mutates_context() {
        let step = SuccessStep;
        let mut ctx = 5;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 6);
    }

    #[tokio::test]
    async fn test_execute_error_returns_failure() {
        let step = FailureStep;
        let mut ctx = 5;
        let result = step.execute(&mut ctx).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_error_message_preserved() {
        let step = FailureStep;
        let mut ctx = 5;
        match step.execute(&mut ctx).await {
            Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "test failure"),
            _ => panic!("Expected StepFailed error"),
        }
    }

    #[test]
    fn test_name_happy_returns_value() {
        let step = SuccessStep;
        assert_eq!(step.name(), "success-step");
    }

    #[test]
    fn test_name_happy_failure_step() {
        let step = FailureStep;
        assert_eq!(step.name(), "failure-step");
    }
}
