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

    #[tokio::test]
    async fn test_noop_step_succeeds() {
        let step = NoopStep;
        let mut ctx: i32 = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
        // Verify name by casting to trait object
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "noop");
    }
}
