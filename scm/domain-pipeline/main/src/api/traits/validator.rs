//! [`Validator`] — validates pipeline configuration and execution constraints.

use crate::api::types::{
    BuilderValidationRequest, ConfigValidationRequest, EnablementRequest, EnablementResponse,
};
use crate::api::PipelineError;

/// Validates pipeline configuration and execution constraints.
#[async_trait::async_trait]
pub trait Validator: Send + Sync {
    /// Validate the pipeline configuration.
    async fn validate(&self, req: ConfigValidationRequest) -> Result<(), PipelineError<String>>;

    /// Validate the configuration embedded in a pipeline builder.
    ///
    /// Default: delegates to [`validate`](Validator::validate) using the builder's config.
    /// The `where Self: Sized` bound keeps the overall trait dyn-compatible.
    async fn validate_builder<Ctx: Send + 'static, E: Send + 'static>(
        &self,
        req: BuilderValidationRequest<'_, Ctx, E>,
    ) -> Result<(), PipelineError<String>>
    where
        Self: Sized,
    {
        self.validate(ConfigValidationRequest {
            config: req.builder.config.clone(),
        })
        .await
    }

    /// Check if this validator is enabled.
    fn is_enabled(
        &self,
        req: EnablementRequest,
    ) -> Result<EnablementResponse, PipelineError<String>>;
}
