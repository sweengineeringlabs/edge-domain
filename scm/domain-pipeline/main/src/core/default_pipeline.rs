//! Concrete implementations for testing and documentation.

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

/// Concrete implementation of Pipeline trait.
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
