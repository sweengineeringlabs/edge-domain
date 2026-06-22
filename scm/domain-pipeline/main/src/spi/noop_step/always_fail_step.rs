//! Test double: step that always fails.

use crate::api::{PipelineError, Step};

/// A step that always fails with a configurable error.
///
/// Used to test error handling and pipeline abort behavior.
#[derive(Clone, Debug)]
pub struct AlwaysFailStep {
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
    async fn test_always_fail_step_new() {
        let step = AlwaysFailStep::new("error");
        let step_ref: &dyn crate::api::Step<()> = &step;
        assert_eq!(step_ref.name(), "always-fail");
    }
}
