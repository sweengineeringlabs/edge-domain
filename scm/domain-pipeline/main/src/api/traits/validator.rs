//! [`Validator`] — validates pipeline configuration and execution constraints.

use crate::api::{PipelineConfig, PipelineError};

/// Validates pipeline configuration and execution constraints.
#[async_trait::async_trait]
pub trait Validator: Send + Sync {
    /// Validate the pipeline configuration.
    async fn validate(&self, config: &PipelineConfig) -> Result<(), PipelineError>;

    /// Check if this validator is enabled.
    fn is_enabled(&self) -> bool;
}
