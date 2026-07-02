//! Test double: step that always succeeds.

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// A step that always succeeds, optionally mutating context.
///
/// Generic over `Ctx`/`E` — works with any pipeline context/error type.
pub(crate) struct AlwaysPassStep<Ctx, E> {
    _phantom: std::marker::PhantomData<fn(Ctx, E)>,
}

impl<Ctx, E> AlwaysPassStep<Ctx, E> {
    /// Create a step that succeeds without mutating context.
    pub(crate) fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Ctx, E> Clone for AlwaysPassStep<Ctx, E> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<Ctx, E> std::fmt::Debug for AlwaysPassStep<Ctx, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AlwaysPassStep").finish()
    }
}

const STEP_NAME: &str = "always-pass";

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step for AlwaysPassStep<Ctx, E> {
    type Ctx = Ctx;
    type ExecutionError = E;

    async fn execute(&self, _req: ContextMutationRequest<'_, Ctx>) -> Result<(), E> {
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

impl<Ctx, E> Default for AlwaysPassStep<Ctx, E> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step: AlwaysPassStep<i32, String> = AlwaysPassStep::new();
        let step_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step;
        let mut ctx: i32 = 0;
        assert!(step_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 0);
    }

    #[tokio::test]
    async fn test_execute_happy_multiple_types() {
        let step_int: AlwaysPassStep<i32, String> = AlwaysPassStep::new();
        let step_int_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step_int;
        let mut ctx_int: i32 = 42;
        assert!(step_int_ref
            .execute(ContextMutationRequest { ctx: &mut ctx_int })
            .await
            .is_ok());

        let step_str: AlwaysPassStep<String, String> = AlwaysPassStep::new();
        let step_str_ref: &dyn Step<Ctx = String, ExecutionError = String> = &step_str;
        let mut ctx_str = "hello".to_string();
        assert!(step_str_ref
            .execute(ContextMutationRequest { ctx: &mut ctx_str })
            .await
            .is_ok());
    }

    #[test]
    fn test_new_happy_creates_instance() {
        let step: AlwaysPassStep<i32, String> = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "always-pass"
        );
    }

    #[test]
    fn test_name_happy_returns_always_pass() {
        let step: AlwaysPassStep<i32, String> = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<Ctx = i32, ExecutionError = String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "always-pass"
        );
    }

    #[tokio::test]
    async fn test_default_happy_equivalent_to_new() {
        let step_new: AlwaysPassStep<i32, String> = AlwaysPassStep::new();
        let step_new_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step_new;
        let step_default: AlwaysPassStep<i32, String> = AlwaysPassStep::default();
        let step_default_ref: &dyn Step<Ctx = i32, ExecutionError = String> = &step_default;
        let mut ctx_new = 0i32;
        let mut ctx_default = 0i32;
        assert!(step_new_ref
            .execute(ContextMutationRequest { ctx: &mut ctx_new })
            .await
            .is_ok());
        assert!(step_default_ref
            .execute(ContextMutationRequest {
                ctx: &mut ctx_default
            })
            .await
            .is_ok());
        assert_eq!(ctx_new, ctx_default);
    }
}
