//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a placeholder when a pipeline position needs a step but no work is required.

use std::marker::PhantomData;

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// Default step: executes without modifying context, always succeeds.
///
/// Generic over `Ctx`/`E` — works with any pipeline context/error type.
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

const DEFAULT_STEP_NAME: &str = "default-step";

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step for DefaultStep<Ctx, E> {
    type Ctx = Ctx;
    type ExecutionError = E;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, crate::api::PipelineError<E>> {
        Ok(StepNameResponse {
            name: DEFAULT_STEP_NAME.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(ctx, 42);
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
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }
}
