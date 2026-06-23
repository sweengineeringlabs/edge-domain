//! [`NoopStep`] — a step that does nothing.

use crate::api::{PipelineError, Step};

/// A step that performs no operation, succeeding without mutation.
#[derive(Clone)]
pub(crate) struct NoopStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for NoopStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "noop-step"
    }
}

/// A concrete, non-generic step for testing.
pub(crate) struct ConcreteStep;

#[async_trait::async_trait]
impl Step<()> for ConcreteStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "concrete-step"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers NoopStep execute
    #[tokio::test]
    async fn test_noop_execute_happy_succeeds() {
        let step = NoopStep;
        let mut ctx = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42); // Context should be unchanged
    }

    /// @covers NoopStep name
    #[test]
    fn test_noop_name_happy_returns_identifier() {
        let step = NoopStep;
        assert_eq!(step.name(), "noop-step");
    }
}
