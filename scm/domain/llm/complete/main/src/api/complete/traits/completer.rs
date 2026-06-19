//! `Completer` — primary HTTP-level LLM completion boundary trait.

use async_trait::async_trait;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::traits::completion_stream::CompletionStream;
use crate::api::complete::types::{CompletionRequest, CompletionResponse, ModelInfo};

/// Primary completion boundary.
///
/// Abstracts a single LLM backend capable of completing a [`CompletionRequest`]
/// in either blocking or streaming mode, and of introspecting its own model list.
#[async_trait]
pub trait Completer: Send + Sync {
    /// Blocking completion — returns the full response once generation finishes.
    async fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, CompleteError>;

    /// Streaming completion — returns a live chunk stream.
    async fn complete_stream(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionStream, CompleteError>;

    /// Model IDs this completer can serve.
    fn supported_models(&self) -> Vec<String>;

    /// Returns `true` if this completer can serve the given model id.
    fn supports(&self, model: &str) -> bool {
        self.supported_models().iter().any(|m| m == model)
    }

    /// Resolve metadata for a single model by id.
    async fn model_info(&self, model: &str) -> Result<ModelInfo, CompleteError>;

    /// List all models available to this completer.
    async fn list_models(&self) -> Result<Vec<ModelInfo>, CompleteError>;

    /// Returns `true` if the given model can be reached right now.
    async fn is_model_available(&self, model: &str) -> bool {
        self.model_info(model).await.is_ok()
    }
}
