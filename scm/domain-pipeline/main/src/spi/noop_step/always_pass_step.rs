//! Test double: step that always succeeds.

use crate::api::{PipelineError, Step};

/// A step that always succeeds, optionally mutating context.
///
/// Used to test successful step execution and context propagation.
#[derive(Clone, Debug)]
pub struct AlwaysPassStep {
    _phantom: std::marker::PhantomData<()>,
}

impl AlwaysPassStep {
    /// Create a step that succeeds without mutating context.
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for AlwaysPassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "always-pass"
    }
}

impl Default for AlwaysPassStep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_always_pass_step_succeeds() {
        let step = AlwaysPassStep::new();
        let mut ctx: i32 = 0;
        assert!(step.execute(&mut ctx).await.is_ok());
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }

    #[tokio::test]
    async fn test_always_pass_step_default() {
        let step = AlwaysPassStep::default();
        let mut ctx: i32 = 0;
        assert!(step.execute(&mut ctx).await.is_ok());
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }

    #[tokio::test]
    async fn test_always_pass_step_new() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<()> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }
}
