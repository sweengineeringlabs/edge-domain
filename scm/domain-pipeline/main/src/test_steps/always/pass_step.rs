//! Test double: step that always succeeds.

use crate::api::{ContextMutationRequest, Step, StepNameRequest, StepNameResponse};

/// A step that always succeeds, optionally mutating context.
///
/// Generic over `E` — works with any pipeline error type.
#[derive(Clone, Debug)]
pub(crate) struct AlwaysPassStep {
    _phantom: std::marker::PhantomData<()>,
}

impl AlwaysPassStep {
    /// Create a step that succeeds without mutating context.
    pub(crate) fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

const STEP_NAME: &str = "always-pass";

#[async_trait::async_trait]
impl<Ctx: Send, E: Send + 'static> Step<Ctx, E> for AlwaysPassStep {
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

impl Default for AlwaysPassStep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn Step<i32, String> = &step;
        let mut ctx: i32 = 0;
        assert!(step_ref
            .execute(ContextMutationRequest { ctx: &mut ctx })
            .await
            .is_ok());
        assert_eq!(ctx, 0);
    }

    #[tokio::test]
    async fn test_execute_happy_multiple_types() {
        let step_int = AlwaysPassStep::new();
        let step_int_ref: &dyn Step<i32, String> = &step_int;
        let mut ctx_int: i32 = 42;
        assert!(step_int_ref
            .execute(ContextMutationRequest { ctx: &mut ctx_int })
            .await
            .is_ok());

        let step_str = AlwaysPassStep::new();
        let step_str_ref: &dyn Step<String, String> = &step_str;
        let mut ctx_str = "hello".to_string();
        assert!(step_str_ref
            .execute(ContextMutationRequest { ctx: &mut ctx_str })
            .await
            .is_ok());
    }

    #[test]
    fn test_new_happy_creates_instance() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<i32, String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "always-pass"
        );
    }

    #[test]
    fn test_name_happy_returns_always_pass() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<i32, String> = &step;
        assert_eq!(
            step_ref.name(StepNameRequest).expect("must succeed").name,
            "always-pass"
        );
    }

    #[tokio::test]
    async fn test_default_happy_equivalent_to_new() {
        let step_new = AlwaysPassStep::new();
        let step_new_ref: &dyn Step<i32, String> = &step_new;
        let step_default = AlwaysPassStep::default();
        let step_default_ref: &dyn Step<i32, String> = &step_default;
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
