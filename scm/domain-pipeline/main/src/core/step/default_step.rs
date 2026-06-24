//! [`DefaultStep`] — no-op step that succeeds without mutation.

use crate::api::{PipelineError, Step};

/// Default step implementation: executes without modifying context.
///
/// Used as fallback or placeholder when a step is optional.
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

    /// @covers DefaultStep::execute
    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step = DefaultStep;
        let mut ctx = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers DefaultStep::name
    #[test]
    fn test_name_happy_returns_identifier() {
        let step = DefaultStep;
        assert_eq!(step.name(), "default-step");
    }
}
