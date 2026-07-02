//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a placeholder when a pipeline position needs a step but no work is required.

use std::marker::PhantomData;

use crate::api::{ContextMutationRequest, PipelineError, Step, StepNameRequest, StepNameResponse};

const DEFAULT_STEP_NAME: &str = "default-step";

/// No-op step: leaves the context unchanged and always succeeds.
pub(crate) struct DefaultStep<Ctx, E>(PhantomData<fn(Ctx, E)>);

impl<Ctx, E> DefaultStep<Ctx, E> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Ctx, E> Clone for DefaultStep<Ctx, E> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<Ctx, E> std::fmt::Debug for DefaultStep<Ctx, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultStep").finish()
    }
}

#[async_trait::async_trait]
impl<Ctx: Send, ExecutionError: Send + 'static> Step for DefaultStep<Ctx, ExecutionError> {
    type Ctx = Ctx;
    type ExecutionError = ExecutionError;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), ExecutionError> {
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, PipelineError<ExecutionError>> {
        Ok(StepNameResponse {
            name: DEFAULT_STEP_NAME.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_happy_constructs_independent_instances() {
        let step_a: DefaultStep<i32, String> = DefaultStep::new();
        let step_b: DefaultStep<i32, String> = DefaultStep::new();
        let step_a: &dyn Step<Ctx = i32, ExecutionError = String> = &step_a;
        let step_b: &dyn Step<Ctx = i32, ExecutionError = String> = &step_b;
        assert_eq!(
            step_a.name(StepNameRequest).expect("must succeed").name,
            step_b.name(StepNameRequest).expect("must succeed").name,
            "two independently constructed instances must report the same identity"
        );
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step: DefaultStep<i32, String> = DefaultStep::new();
        let step: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        let mut ctx = 42;
        assert!(step
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_edge_idempotent() {
        let step: DefaultStep<i32, String> = DefaultStep::new();
        let step: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        let mut ctx = 42;
        assert!(step
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert!(step
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: name
    #[test]
    fn test_name_happy_returns_identifier() {
        let step: DefaultStep<i32, String> = DefaultStep::new();
        let step: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }

    /// @covers: name
    #[test]
    fn test_name_edge_stable_across_context_types() {
        let step: DefaultStep<(), String> = DefaultStep::new();
        let step: &dyn Step<Ctx = (), ExecutionError = String> = &step;
        assert_eq!(
            step.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }
}
