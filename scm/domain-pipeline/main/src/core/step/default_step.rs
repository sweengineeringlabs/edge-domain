//! [`DefaultStep`] — no-op step that succeeds without mutation.
//!
//! Used as a safe fallback when a step is optional or not yet implemented.
//! Always succeeds, never modifies context. Safe for use in any pipeline context type.

use crate::api::{PipelineError, Step};

/// Default step implementation: executes without modifying context.
///
/// This step always succeeds and performs no mutations. It is idempotent
/// and safe to execute multiple times on the same context.
/// Use when a pipeline position needs a step but no actual work is required.
#[derive(Clone)]
pub(crate) struct DefaultStep;

#[async_trait::async_trait]
impl<Ctx: Send> Step<Ctx> for DefaultStep {
    async fn execute(&self, _ctx: &mut Ctx) -> Result<(), PipelineError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers DefaultStep::execute
    #[tokio::test]
    async fn test_execute_happy_succeeds() {
        let step = DefaultStep;
        let mut ctx = 42;
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers DefaultStep::execute
    #[tokio::test]
    async fn test_execute_edge_idempotent() {
        let step = DefaultStep;
        let mut ctx = 42;
        // First execution
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
        // Second execution — should produce identical result
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
        // Third execution — consistent behavior
        assert!(step.execute(&mut ctx).await.is_ok());
        assert_eq!(ctx, 42);
    }

    /// @covers DefaultStep::name
    #[test]
    fn test_name_happy_uses_type_name() {
        let step = DefaultStep;
        assert_eq!(step.name(), "core::step::default_step::DefaultStep");
    }

    /// @covers DefaultStep::name
    #[test]
    fn test_name_edge_consistent_across_calls() {
        let step = DefaultStep;
        let name1 = step.name();
        let name2 = step.name();
        assert_eq!(name1, name2);
    }
}
