//! Test double: step that always fails.

use crate::api::{PipelineError, Step};

/// A step that always fails with a configurable error.
///
/// Used to test error handling and pipeline abort behavior.
#[derive(Clone, Debug)]
pub(crate) struct AlwaysFailStep {
    error_msg: String,
}

impl AlwaysFailStep {
    /// Create a step that always fails with the given error message.
    pub(crate) fn new(error_msg: impl Into<String>) -> Self {
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
    async fn test_execute_error_returns_failure() {
        let step = AlwaysFailStep::new("test error");
        let mut ctx: i32 = 0;
        let result = step.execute(&mut ctx).await;
        assert!(result.is_err());
    }

    /// @covers: AlwaysFailStep::execute
    #[tokio::test]
    async fn test_execute_error_preserves_message() {
        let step = AlwaysFailStep::new("custom failure");
        let mut ctx: i32 = 0;
        match step.execute(&mut ctx).await {
            Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "custom failure"),
            _ => panic!("expected StepFailed"),
        }
    }

    #[tokio::test]
    async fn test_new_happy_creates_instance() {
        let step = AlwaysFailStep::new("error");
        let mut ctx = ();
        assert!(step.execute(&mut ctx).await.is_err());
    }

    #[test]
    fn test_name_happy_returns_always_fail() {
        let step = AlwaysFailStep::new("test");
        let step_ref: &dyn crate::api::Step<i32> = &step;
        assert_eq!(step_ref.name(), "always-fail");
    }

    /// @covers: AlwaysFailStep::execute
    #[tokio::test]
    async fn test_execute_error_different_messages() {
        let step1 = AlwaysFailStep::new("error1");
        let step2 = AlwaysFailStep::new("error2");
        let mut ctx = 0;

        match step1.execute(&mut ctx).await {
            Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "error1"),
            _ => panic!("expected StepFailed"),
        }

        match step2.execute(&mut ctx).await {
            Err(PipelineError::StepFailed(msg)) => assert_eq!(msg, "error2"),
            _ => panic!("expected StepFailed"),
        }
    }
}
