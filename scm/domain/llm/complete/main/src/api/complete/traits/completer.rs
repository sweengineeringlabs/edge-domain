//! `Completer` — primary HTTP-level LLM completion boundary trait.

use async_trait::async_trait;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{
    CompleteRequest, CompleterHealthCheckRequest, CompleterHealthCheckResponse, CompletionResponse,
    CompletionStreamRequest, CompletionStreamResponse, ListModelsRequest, ListModelsResponse,
    ModelAvailabilityRequest, ModelAvailabilityResponse, ModelInfoRequest, ModelInfoResponse,
    ModelSupportRequest, ModelSupportResponse, SupportedModelsRequest, SupportedModelsResponse,
};

/// Primary completion boundary.
///
/// Abstracts a single LLM backend capable of completing a [`CompletionRequest`](crate::api::complete::types::CompletionRequest)
/// in either blocking or streaming mode, and of introspecting its own model list.
#[async_trait]
pub trait Completer: Send + Sync {
    /// Blocking completion — returns the full response once generation finishes.
    async fn complete(&self, req: CompleteRequest<'_>)
        -> Result<CompletionResponse, CompleteError>;

    /// Streaming completion — returns a live chunk stream.
    async fn complete_stream(
        &self,
        req: CompletionStreamRequest<'_>,
    ) -> Result<CompletionStreamResponse, CompleteError>;

    /// Model IDs this completer can serve.
    fn supported_models(
        &self,
        req: SupportedModelsRequest,
    ) -> Result<SupportedModelsResponse, CompleteError>;

    /// Returns `true` if this completer can serve the given model id.
    fn supports(
        &self,
        req: ModelSupportRequest<'_>,
    ) -> Result<ModelSupportResponse, CompleteError> {
        let supported = self
            .supported_models(SupportedModelsRequest)?
            .models
            .iter()
            .any(|m| m == req.model);
        Ok(ModelSupportResponse { supported })
    }

    /// Resolve metadata for a single model by id.
    async fn model_info(
        &self,
        req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError>;

    /// List all models available to this completer.
    async fn list_models(
        &self,
        req: ListModelsRequest,
    ) -> Result<ListModelsResponse, CompleteError>;

    /// Returns `true` if the given model can be reached right now.
    async fn is_model_available(
        &self,
        req: ModelAvailabilityRequest<'_>,
    ) -> Result<ModelAvailabilityResponse, CompleteError> {
        let available = self
            .model_info(ModelInfoRequest { model: req.model })
            .await
            .is_ok();
        Ok(ModelAvailabilityResponse { available })
    }

    /// Returns `true` if the completer is healthy and can process requests.
    ///
    /// Default implementation probes by attempting to list models.
    /// Implementations may override with a lighter probe (e.g., a dedicated health endpoint).
    async fn health_check(
        &self,
        _req: CompleterHealthCheckRequest,
    ) -> Result<CompleterHealthCheckResponse, CompleteError> {
        let healthy = self.list_models(ListModelsRequest).await.is_ok();
        Ok(CompleterHealthCheckResponse { healthy })
    }
}
