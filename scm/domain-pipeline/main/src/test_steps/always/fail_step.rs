//! Test double: step that always fails.

use std::marker::PhantomData;

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// A step that always fails with a configurable error of type `E`.
///
/// Used to test error handling and pipeline abort behavior.
pub(crate) struct AlwaysFailStep<Ctx, E> {
    error: E,
    _phantom: PhantomData<fn(Ctx)>,
}

impl<Ctx, E: Clone + Send + Sync + std::fmt::Debug + 'static> AlwaysFailStep<Ctx, E> {
    /// Create a step that always fails with the given error.
    pub(crate) fn new(error: E) -> Self {
        Self {
            error,
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, E: Clone> Clone for AlwaysFailStep<Ctx, E> {
    fn clone(&self) -> Self {
        Self {
            error: self.error.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, E: std::fmt::Debug> std::fmt::Debug for AlwaysFailStep<Ctx, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AlwaysFailStep")
            .field("error", &self.error)
            .finish()
    }
}

const STEP_NAME: &str = "always-fail";

#[async_trait::async_trait]
impl<Ctx: Send, E: Clone + Send + Sync + std::fmt::Debug + 'static> Step
    for AlwaysFailStep<Ctx, E>
{
    type Ctx = Ctx;
    type ExecutionError = E;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Err(self.error.clone())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, crate::api::PipelineError<E>> {
        Ok(StepNameResponse {
            name: STEP_NAME.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_error_returns_failure() {
        let step = AlwaysFailStep::new("test error".to_string());
        let mut ctx: i32 = 0;
        let result = step.execute(ContextMutationRequest { ctx: &mut ctx }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_error_preserves_error_value() {
        let step = AlwaysFailStep::new("custom failure".to_string());
        let mut ctx: i32 = 0;
        match step.execute(ContextMutationRequest { ctx: &mut ctx }).await {
            Err(msg) => assert_eq!(msg, "custom failure"),
            _ => panic!("expected Err"),
        }
    }

    #[tokio::test]
    async fn test_new_happy_creates_instance() {
        let step = AlwaysFailStep::new("error".to_string());
        let mut ctx = ();
        assert!(step
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_err());
    }

    #[test]
    fn test_name_happy_returns_always_fail() {
        let step: AlwaysFailStep<i32, String> = AlwaysFailStep::new("test".to_string());
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "always-fail"
        );
    }

    #[tokio::test]
    async fn test_execute_error_different_values() {
        let step1 = AlwaysFailStep::new("error1".to_string());
        let step2 = AlwaysFailStep::new("error2".to_string());
        let mut ctx = 0;
        match step1
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
        {
            Err(msg) => assert_eq!(msg, "error1"),
            _ => panic!("expected Err"),
        }
        match step2
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
        {
            Err(msg) => assert_eq!(msg, "error2"),
            _ => panic!("expected Err"),
        }
    }

    #[test]
    fn test_new_happy_stores_error() {
        let step: AlwaysFailStep<i32, String> = AlwaysFailStep::new("test error".to_string());
        assert_eq!(step.error, "test error");
    }
}
