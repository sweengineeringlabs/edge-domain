//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a placeholder when a pipeline position needs a step but no work is required.

use crate::api::{ContextMutationRequest, PipelineError, Step, StepNameRequest, StepNameResponse};

const DEFAULT_STEP_NAME: &str = "default-step";

/// No-op step: leaves the context unchanged and always succeeds.
#[derive(Clone, Debug)]
pub(crate) struct DefaultStep;

#[async_trait::async_trait]
impl<Ctx: Send, ExecutionError: Send + 'static> Step<Ctx, ExecutionError> for DefaultStep {
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
        assert!(step
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: name
    #[test]
    fn test_name_happy_returns_identifier() {
        let step: &dyn Step<i32, String> = &DefaultStep;
        assert_eq!(
            step.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }

    /// @covers: name
    #[test]
    fn test_name_edge_stable_across_context_types() {
        let step: &dyn Step<(), String> = &DefaultStep;
        assert_eq!(
            step.name(StepNameRequest).expect("must succeed").name,
            "default-step"
        );
    }
}
