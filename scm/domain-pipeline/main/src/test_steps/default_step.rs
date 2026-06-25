//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a placeholder when a pipeline position needs a step but no work is required.

use crate::api::{PipelineError, Step};

/// Default step: executes without modifying context, always succeeds.
#[derive(Clone)]
pub(crate) struct DefaultStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for DefaultStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "default-step"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step = DefaultStep;
        let mut ctx = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: execute
    #[tokio::test]
    async fn test_execute_edge_idempotent() {
        let step = DefaultStep;
        let mut ctx = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: name
    #[test]
    fn test_name_happy_returns_identifier() {
        let step = DefaultStep;
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "default-step");
    }
}
