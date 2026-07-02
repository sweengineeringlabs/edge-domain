//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a placeholder when a pipeline position needs a step but no work is required.

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// Default step: executes without modifying context, always succeeds.
///
/// Generic over `E` — works with any pipeline error type.
#[derive(Clone)]
pub(crate) struct DefaultStep;

const DEFAULT_STEP_NAME: &str = "default-step";

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for DefaultStep {
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
        let step: &dyn Step<i32, String> = &DefaultStep;
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
        let step: &dyn Step<i32, String> = &DefaultStep;
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
        let step = DefaultStep;
        let step_ref: &dyn crate::api::Step<i32, String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }
}
