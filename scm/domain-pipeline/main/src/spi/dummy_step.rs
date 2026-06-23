//! Test double: dummy step for builder pattern testing.

use crate::api::{PipelineError, Step};

/// A step that does nothing, used for testing builder pattern logic.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DummyStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for DummyStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "dummy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step = DummyStep;
        let mut ctx: i32 = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42); // Verify no mutation
    }

    #[test]
    fn test_name_happy_returns_dummy() {
        let step = DummyStep;
        let step_ref: &dyn Step<i32> = &step;
        assert_eq!(step_ref.name(), "dummy");
    }
}
