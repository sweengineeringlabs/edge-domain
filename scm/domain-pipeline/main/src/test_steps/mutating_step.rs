//! Test double: step that mutates context.

use crate::api::Step;

/// A step that mutates context and succeeds.
///
/// Generic over `E` — works with any pipeline error type.
#[derive(Clone, Debug)]
pub(crate) struct MutatingStep<F> {
    mutate_fn: F,
}

impl<F> MutatingStep<F> {
    /// Create a step that applies the given mutation.
    pub(crate) fn new(mutate_fn: F) -> Self {
        Self { mutate_fn }
    }
}

const STEP_NAME: &str = "mutating";

#[async_trait::async_trait]
impl<Ctx: Send, F: Fn(&mut Ctx) + Send + Sync, E: Send + 'static> Step<Ctx, E>
    for MutatingStep<F>
{
    async fn execute(&self, ctx: &mut Ctx) -> Result<(), E> {
        (self.mutate_fn)(ctx);
        Ok(())
    }

    fn name(&self) -> &str {
        STEP_NAME
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_happy_applies_mutation_int() {
        let step = MutatingStep::new(|ctx: &mut i32| *ctx += 10);
        let step_ref: &dyn Step<i32, String> = &step;
        let mut ctx = 5;
        assert!(step_ref.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 15);
    }

    #[tokio::test]
    async fn test_execute_happy_applies_mutation_string() {
        let step = MutatingStep::new(|ctx: &mut String| ctx.push_str("!"));
        let step_ref: &dyn Step<String, String> = &step;
        let mut ctx = "hello".to_string();
        assert!(step_ref.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, "hello!");
    }

    #[tokio::test]
    async fn test_new_happy_creates_instance() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn Step<i32, String> = &step;
        let mut ctx = 0;
        assert!(step_ref.execute(&mut ctx).await.is_ok());
    }

    #[test]
    fn test_name_happy_returns_mutating() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn crate::api::Step<i32, String> = &step;
        assert_eq!(step_ref.name(), "mutating");
    }

    #[tokio::test]
    async fn test_execute_happy_multiple_mutations() {
        let step1 = MutatingStep::new(|ctx: &mut i32| *ctx *= 2);
        let step1_ref: &dyn Step<i32, String> = &step1;
        let step2 = MutatingStep::new(|ctx: &mut i32| *ctx += 5);
        let step2_ref: &dyn Step<i32, String> = &step2;

        let mut ctx = 10;
        assert!(step1_ref.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 20);

        assert!(step2_ref.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 25);
    }

    #[test]
    fn test_new_happy_stores_closure() {
        let step = MutatingStep::new(|_ctx: &mut i32| {});
        let step_ref: &dyn crate::api::Step<i32, String> = &step;
        assert_eq!(step_ref.name(), "mutating");
    }
}
