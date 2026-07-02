//! Test double: step that mutates context.

use std::marker::PhantomData;

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// A step that mutates context and succeeds.
///
/// Generic over `Ctx`/`E` — works with any pipeline context/error type.
pub(crate) struct MutatingStep<Ctx, E, F> {
    mutate_fn: F,
    _phantom: PhantomData<fn(Ctx, E)>,
}

impl<Ctx, E, F> MutatingStep<Ctx, E, F> {
    /// Create a step that applies the given mutation.
    pub(crate) fn new(mutate_fn: F) -> Self {
        Self {
            mutate_fn,
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, E, F: Clone> Clone for MutatingStep<Ctx, E, F> {
    fn clone(&self) -> Self {
        Self {
            mutate_fn: self.mutate_fn.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<Ctx, E, F> std::fmt::Debug for MutatingStep<Ctx, E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutatingStep").finish()
    }
}

const STEP_NAME: &str = "mutating";

#[async_trait::async_trait]
impl<Ctx: Send, F: Fn(&mut Ctx) + Send + Sync, E: Send + 'static> Step for MutatingStep<Ctx, E, F> {
    type Ctx = Ctx;
    type ExecutionError = E;

    async fn execute(&self, req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
        (self.mutate_fn)(req.ctx);
        Ok(())
    }

    fn name(
        &self,
        _req: StepNameRequest,
    ) -> Result<StepNameResponse, crate::api::PipelineError<E>> {
        Ok(StepNameResponse {
            name: STEP_NAME.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_happy_applies_mutation_int() {
        let step = MutatingStep::new(|ctx: &mut i32| *ctx += 10);
        let step_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        let mut ctx = 5;
        assert!(step_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 15);
    }

    #[tokio::test]
    async fn test_execute_happy_applies_mutation_string() {
        let step = MutatingStep::new(|ctx: &mut String| ctx.push('!'));
        let step_ref: &dyn Step<Ctx = String, ExecutionError = String> = &step;
        let mut ctx = "hello".to_string();
        assert!(step_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, "hello!");
    }

    #[tokio::test]
    async fn test_new_happy_creates_instance() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        let mut ctx = 0;
        assert!(step_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
    }

    #[test]
    fn test_name_happy_returns_mutating() {
        let step = MutatingStep::new(|_x: &mut i32| {});
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "mutating"
        );
    }

    #[tokio::test]
    async fn test_execute_happy_multiple_mutations() {
        let step1 = MutatingStep::new(|ctx: &mut i32| *ctx *= 2);
        let step1_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step1;
        let step2 = MutatingStep::new(|ctx: &mut i32| *ctx += 5);
        let step2_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step2;

        let mut ctx = 10;
        assert!(step1_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 20);

        assert!(step2_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 25);
    }

    #[test]
    fn test_new_happy_stores_closure() {
        let step = MutatingStep::new(|_ctx: &mut i32| {});
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "mutating"
        );
    }
}
