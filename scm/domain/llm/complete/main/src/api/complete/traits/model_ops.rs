//! `ModelOps` — model metadata factory, separated for theme cohesion.

use async_trait::async_trait;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::ModelInfo;

/// Model metadata factory and async resolver.
#[async_trait]
pub trait ModelOps: Send + Sync {
    /// Resolve metadata for a single model by id.
    async fn find_model(&self, name: &str) -> Result<ModelInfo, CompleteError>;

    /// Construct a [`ModelInfo`] with all capability flags set to false.
    fn create_model_info(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
        context_window: u32,
    ) -> ModelInfo
    where
        Self: Sized,
    {
        ModelInfo::new(id, name, provider, context_window)
    }
}
