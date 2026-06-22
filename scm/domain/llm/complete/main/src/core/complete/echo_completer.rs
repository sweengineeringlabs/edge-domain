//! `Completer`, `Processor`, `CompleterHandler`, and `Validator` impls for `EchoCompleter`.

use async_trait::async_trait;
use futures::stream;

use crate::api::{
    CompleteError, Completer, CompleterHandler, CompletionRequest, CompletionResponse,
    CompletionStream, EchoCompleter, FinishReason, MessageContent, ModelInfo, Processor,
    StreamChunk, StreamDelta, Validator,
};

impl EchoCompleter {
    fn last_user_text(&self, req: &CompletionRequest) -> String {
        req.messages
            .iter()
            .rev()
            .find_map(|m| match &m.content {
                MessageContent::Text(t) => Some(t.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }
}

#[async_trait]
impl Completer for EchoCompleter {
    async fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, CompleteError> {
        let content = self.last_user_text(request);
        Ok(CompletionResponse::text("echo-1", &request.model, content))
    }

    async fn complete_stream(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionStream, CompleteError> {
        let content = self.last_user_text(request);
        let chunk = StreamChunk::terminal("echo-1", StreamDelta::text(content), FinishReason::Stop);
        Ok(Box::pin(stream::once(async move { Ok(chunk) })))
    }

    fn supported_models(&self) -> Vec<String> {
        vec!["echo".to_string()]
    }

    async fn model_info(&self, model: &str) -> Result<ModelInfo, CompleteError> {
        if model == "echo" {
            Ok(ModelInfo::new("echo", "Echo Model", "echo", 4096))
        } else {
            Err(CompleteError::ModelNotFound(model.to_string()))
        }
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>, CompleteError> {
        Ok(vec![ModelInfo::new("echo", "Echo Model", "echo", 4096)])
    }
}

#[async_trait]
impl Processor for EchoCompleter {
    async fn process(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, CompleteError> {
        self.complete(request).await
    }
}

impl CompleterHandler for EchoCompleter {}

impl Validator for EchoCompleter {
    fn validate(&self, request: &CompletionRequest) -> Result<(), CompleteError> {
        if request.model.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "model cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
