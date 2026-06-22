//! Test double: step that mutates context.

use crate::api::{PipelineError, Step};

/// A step that mutates context and succeeds.
///
/// Generic version for more complex test scenarios.
#[derive(Clone, Debug)]
pub struct MutatingStep<F> {
    mutate_fn: F,
}

impl<F> MutatingStep<F> {
    /// Create a step that applies the given mutation.
    pub fn new(mutate_fn: F) -> Self {
        Self { mutate_fn }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send, F: Fn(&mut Ctx) + Send + Sync> Step<Ctx> for MutatingStep<F> {
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), PipelineError> {
        (self.mutate_fn)(ctx);
        Ok(())
    }

    fn name(&self) -> &str {
        "mutating"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_always_pass_step_with_mutation() {
        let step = MutatingStep::new(|ctx: &mut i32| *ctx += 10);
        let mut ctx = 5;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 15);
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "mutating");
    }

    #[tokio::test]
    async fn test_mutating_step_applies_mutation() {
        let step = MutatingStep::new(|ctx: &mut String| ctx.push_str("!"));
        let mut ctx = "hello".to_string();
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, "hello!");
    }

    #[tokio::test]
    async fn test_mutating_step_new() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "mutating");
    }
}
