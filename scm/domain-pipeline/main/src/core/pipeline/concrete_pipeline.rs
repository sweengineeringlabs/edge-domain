//! [`ConcretePipeline`] — a concrete, non-generic pipeline for testing.

use crate::api::{Pipeline, PipelineConfig, PipelineError};

/// A concrete, non-generic pipeline for testing.
pub(crate) struct ConcretePipeline {
    config: PipelineConfig,
}

impl ConcretePipeline {
    /// Create a new concrete pipeline.
    pub(crate) fn new() -> Self {
        Self {
            config: PipelineConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl Pipeline<()> for ConcretePipeline {
    async fn execute(&self, _ctx: &mut ()) -> Result<(), PipelineError> {
        Ok(())
    }

    fn step_count(&self) -> usize {
        0
    }

    fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers ConcretePipeline::new
    #[test]
    fn test_new_happy_creates_with_default_config() {
        let pipeline = ConcretePipeline::new();
        assert_eq!(pipeline.step_count(), 0);
        assert!(pipeline.config().timeout_per_step.is_none());
    }

    /// @covers ConcretePipeline::new
    #[test]
    fn test_new_edge_config_always_default() {
        let p1 = ConcretePipeline::new();
        let p2 = ConcretePipeline::new();
        assert_eq!(p1.config().abort_on_error, p2.config().abort_on_error);
    }

    /// @covers ConcretePipeline::new
    #[test]
    fn test_new_error_multiple_calls_independent() {
        let pipeline = ConcretePipeline::new();
        let step_count1 = pipeline.step_count();
        let step_count2 = pipeline.step_count();
        assert_eq!(step_count1, step_count2);
    }
}
