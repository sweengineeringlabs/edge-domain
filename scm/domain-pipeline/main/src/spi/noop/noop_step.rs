//! Test double: step that does nothing.

use crate::api::{PipelineError, Step};

/// A step that does nothing and always succeeds.
///
/// Used in tests to fill pipelines without side effects.
#[derive(Clone, Debug)]
pub struct NoopStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for NoopStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "noop"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: NoopStep::execute
    #[tokio::test]
    async fn test_execute_happy_no_mutation() {
        let step = NoopStep;
        let mut ctx: i32 = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers: NoopStep::execute
    #[tokio::test]
    async fn test_execute_happy_string_context() {
        let step = NoopStep;
        let mut ctx = "hello".to_string();
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, "hello");
    }

    /// @covers: Step::name
    #[tokio::test]
    async fn test_name_happy_returns_noop() {
        let step = NoopStep;
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "noop");
    }

    /// @covers: NoopStep clone
    #[tokio::test]
    async fn test_clone_happy_works() {
        let step = NoopStep;
        let step_cloned = step.clone();
        let mut ctx = 0;
        assert!(step_cloned.execute(&mut ctx).await.is_ok());
    }
}
