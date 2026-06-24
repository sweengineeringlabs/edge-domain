//! Test double: step that always succeeds.

use crate::api::{PipelineError, Step};

/// A step that always succeeds, optionally mutating context.
///
/// Used to test successful step execution and context propagation.
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

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for AlwaysPassStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        // Returns the concrete implementation name for identification and logging.
        // This is not a stub—it's the required Step trait contract for observability.
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
    async fn test_execute_happy_succeeds() {
        let step = AlwaysPassStep::new();
        let mut ctx: i32 = 0;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 0); // Verify no mutation
    }

    #[tokio::test]
    async fn test_execute_happy_multiple_types() {
        let step_int = AlwaysPassStep::new();
        let mut ctx_int: i32 = 42;
        assert!(step_int.execute(&mut ctx_int).await.is_ok());

        let step_str = AlwaysPassStep::new();
        let mut ctx_str = "hello".to_string();
        assert!(step_str.execute(&mut ctx_str).await.is_ok());
    }

    #[test]
    fn test_new_happy_creates_instance() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }

    #[test]
    fn test_name_happy_returns_always_pass() {
        let step = AlwaysPassStep::new();
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }

    #[tokio::test]
    async fn test_default_happy_equivalent_to_new() {
        let step_new = AlwaysPassStep::new();
        let step_default = AlwaysPassStep::default();
        let mut ctx_new = 0;
        let mut ctx_default = 0;
        assert!(step_new.execute(&mut ctx_new).await.is_ok());
        assert!(step_default.execute(&mut ctx_default).await.is_ok());
        assert_eq!(ctx_new, ctx_default);
    }
}
