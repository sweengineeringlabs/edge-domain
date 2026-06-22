//! Test doubles for [`Step`](crate::api::Step) trait.

use crate::api::{PipelineError, Step};

/// A step that does nothing and always succeeds.
///
/// Used in tests to fill pipelines without side effects.
#[derive(Clone, Debug)]
pub(crate) struct NoopStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for NoopStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "noop"
    }
}

/// A step that always succeeds, optionally mutating context.
///
/// Used to test successful step execution and context propagation.
#[derive(Clone, Debug)]
pub(crate) struct AlwaysPassStep {
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

/// A step that mutates context and succeeds.
///
/// Generic version for more complex test scenarios.
#[derive(Clone, Debug)]
pub(crate) struct MutatingStep<F> {
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

/// A step that always fails with a configurable error.
///
/// Used to test error handling and pipeline abort behavior.
#[derive(Clone, Debug)]
pub(crate) struct AlwaysFailStep {
    error_msg: String,
}

impl AlwaysFailStep {
    /// Create a step that always fails with the given error message.
    pub fn new(error_msg: impl Into<String>) -> Self {
        Self {
            error_msg: error_msg.into(),
        }
    }
}

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for AlwaysFailStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Err(PipelineError::StepFailed(self.error_msg.clone()))
    }

    fn name(&self) -> &str {
        "always-fail"
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

    #[tokio::test]
    async fn test_always_pass_step_succeeds() {
        let step = AlwaysPassStep::new();
        let mut ctx: i32 = 0;
        assert!(step.execute(&mut ctx).await.is_ok());
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-pass");
    }

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
    async fn test_always_fail_step_fails() {
        let step = AlwaysFailStep::new("test error");
        let mut ctx: i32 = 0;
        let result = step.execute(&mut ctx).await;
        assert!(result.is_err());
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-fail");
    }

    #[tokio::test]
    async fn test_always_fail_step_error_message() {
        let step = AlwaysFailStep::new("custom failure");
        let mut ctx: i32 = 0;
        match step.execute(&mut ctx).await {
            Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "custom failure"),
            _ => panic!("expected StepFailed"),
        }
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
    async fn test_mutating_step_applies_mutation() {
        let step = MutatingStep::new(|ctx: &mut String| ctx.push_str("!"));
        let mut ctx = "hello".to_string();
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, "hello!");
    }
}
