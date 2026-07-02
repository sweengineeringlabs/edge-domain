//! [`ConfigValidationRequest`] — request wrapping a pipeline configuration to validate.

use crate::api::PipelineConfig;

/// Request to validate a pipeline configuration.
pub struct ConfigValidationRequest {
    /// The configuration to validate.
    pub config: PipelineConfig,
}
