//! [`ConcreteStep`] — a concrete, non-generic step for testing.

use crate::api::{PipelineError, Step};

/// A concrete, non-generic step for testing.
pub(crate) struct ConcreteStep;

#[async_trait::async_trait]
impl Step<()> for ConcreteStep {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "concrete-step"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers ConcreteStep::execute
    #[tokio::test]
    async fn test_concrete_execute_happy_succeeds() {
        let step = ConcreteStep;
        let mut ctx = ();
        assert!(step.execute(&mut ctx).await.is_ok());
    }

    /// @covers ConcreteStep::name
    #[test]
    fn test_concrete_name_happy_returns_identifier() {
        let step = ConcreteStep;
        assert_eq!(
            <ConcreteStep as Step<()>>::name(&step),
            "concrete-step"
        );
    }
}
