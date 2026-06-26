//! [`Step<Ctx, E>`] — single composable operation in a pipeline.

use super::super::error::step_error::StepError;

/// A single composable step in a [`Pipeline`](super::pipeline::Pipeline).
///
/// Steps execute sequentially, each receiving a mutable reference to shared context.
/// On success (`Ok(())`), context mutations are applied and execution continues.
/// On failure (`Err(E)`), the engine wraps the error in a
/// [`StepError<E>`](crate::StepError) (adding the step name) and either aborts
/// or continues depending on `abort_on_error` config.
///
/// `E` is the consumer's domain error type. All steps in one pipeline must share `E`.
///
/// # Invariant
///
/// A step must be idempotent over the context — calling it twice with the same context
/// must produce the same result or error.
#[async_trait::async_trait]
pub trait Step<Ctx, E>: Send + Sync
where
    E: Send + 'static,
{
    /// Execute this step, mutating the context in-place.
    ///
    /// Returns `Ok(())` on success or `Err(E)` on failure.
    /// The context may be partially mutated before an error; callers must not assume rollback.
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), E>;

    /// Human-readable name for this step (logging, debugging, observability).
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// Wrap `cause` in a [`StepError`] annotated with this step's name.
    ///
    /// Convenience for steps that need to attach context before returning `Err`.
    fn fail_with(&self, cause: E) -> StepError<E> {
        StepError {
            step_name: self.name().to_string(),
            cause,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SuccessStep;

    #[async_trait::async_trait]
    impl Step<i32, String> for SuccessStep {
        async fn execute(&self, ctx: &mut i32) -> Result<(), String> {
            *ctx += 1;
            Ok(())
        }

        fn name(&self) -> &str {
            "success-step"
        }
    }

    struct FailureStep;

    #[async_trait::async_trait]
    impl Step<i32, String> for FailureStep {
        async fn execute(&self, _ctx: &mut i32) -> Result<(), String> {
            Err("test failure".to_string())
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
            Err(msg) => assert_eq!(msg, "test failure"),
            _ => panic!("Expected error"),
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

    #[test]
    fn test_fail_with_happy_wraps_cause_with_step_name() {
        let step = FailureStep;
        let err = step.fail_with("injected error".to_string());
        assert_eq!(err.step_name, "failure-step");
        assert_eq!(err.cause, "injected error");
    }
}
