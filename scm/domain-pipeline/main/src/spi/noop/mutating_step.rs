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

    /// @covers: MutatingStep::execute
    #[tokio::test]
    async fn test_execute_happy_applies_mutation_int() {
        let step = MutatingStep::new(|ctx: &mut i32| *ctx += 10);
        let mut ctx = 5;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 15);
    }

    /// @covers: MutatingStep::execute
    #[tokio::test]
    async fn test_execute_happy_applies_mutation_string() {
        let step = MutatingStep::new(|ctx: &mut String| ctx.push_str("!"));
        let mut ctx = "hello".to_string();
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, "hello!");
    }

    /// @covers: MutatingStep::new
    #[tokio::test]
    async fn test_new_happy_creates_instance() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let mut ctx = 0;
        assert!(step.execute(&mut ctx).await.is_ok());
    }

    /// @covers: Step::name
    #[test]
    fn test_name_happy_returns_mutating() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "mutating");
    }

    /// @covers: MutatingStep::execute
    #[tokio::test]
    async fn test_execute_happy_multiple_mutations() {
        let step1 = MutatingStep::new(|ctx: &mut i32| *ctx *= 2);
        let step2 = MutatingStep::new(|ctx: &mut i32| *ctx += 5);

        let mut ctx = 10;
        assert!(step1.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 20);

        assert!(step2.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 25);
    }

    /// @covers: MutatingStep::execute
    #[tokio::test]
    async fn test_execute_happy_complex_type() {
        struct Counter {
            count: usize,
        }

        let step = MutatingStep::new(|ctx: &mut Counter| ctx.count += 1);
        let mut ctx = Counter { count: 0 };
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx.count, 1);
    }
}
