//! [`PipelineConfigResponse`] — wraps a pipeline's configuration.

use super::PipelineConfig;

/// Response carrying a pipeline's execution configuration.
#[derive(Debug, Clone)]
pub struct PipelineConfigResponse {
    /// The pipeline's configuration.
    pub config: PipelineConfig,
}
