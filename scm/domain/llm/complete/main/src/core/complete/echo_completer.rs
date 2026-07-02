//! `Completer`, `Processor`, `CompleterHandler`, and `Validator` impls for `EchoCompleter`.

use async_trait::async_trait;
use futures::stream;

use crate::api::{
    CompleteError, CompleteRequest, Completer, CompleterHandler, CompletionRequest,
    CompletionResponse, CompletionStreamRequest, CompletionStreamResponse, EchoCompleter,
    FinishReason, ListModelsRequest, ListModelsResponse, MessageContent, ModelInfo,
    ModelInfoRequest, ModelInfoResponse, ProcessingRequest, Processor, StreamChunk, StreamDelta,
    SupportedModelsRequest, SupportedModelsResponse, ValidationRequest, Validator,
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
        req: CompleteRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        let content = self.last_user_text(req.request);
        Ok(CompletionResponse::text(
            "echo-1",
            &req.request.model,
            content,
        ))
    }

    async fn complete_stream(
        &self,
        req: CompletionStreamRequest<'_>,
    ) -> Result<CompletionStreamResponse, CompleteError> {
        let content = self.last_user_text(req.request);
        let chunk = StreamChunk::terminal("echo-1", StreamDelta::text(content), FinishReason::Stop);
        Ok(CompletionStreamResponse {
            stream: Box::pin(stream::once(async move { Ok(chunk) })),
        })
    }

    fn supported_models(
        &self,
        _req: SupportedModelsRequest,
    ) -> Result<SupportedModelsResponse, CompleteError> {
        Ok(SupportedModelsResponse {
            models: vec!["echo".to_string()],
        })
    }

    async fn model_info(
        &self,
        req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError> {
        if req.model == "echo" {
            Ok(ModelInfoResponse {
                info: Box::new(ModelInfo::new("echo", "Echo Model", "echo", 4096)),
            })
        } else {
            Err(CompleteError::ModelNotFound(req.model.to_string()))
        }
    }

    async fn list_models(
        &self,
        _req: ListModelsRequest,
    ) -> Result<ListModelsResponse, CompleteError> {
        Ok(ListModelsResponse {
            models: vec![ModelInfo::new("echo", "Echo Model", "echo", 4096)],
        })
    }
}

#[async_trait]
impl Processor for EchoCompleter {
    async fn process(
        &self,
        req: ProcessingRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        self.complete(CompleteRequest {
            request: req.request,
        })
        .await
    }
}

impl CompleterHandler for EchoCompleter {}

impl Validator for EchoCompleter {
    fn validate(&self, req: ValidationRequest<'_>) -> Result<(), CompleteError> {
        if req.request.model.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "model cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    fn sample_request() -> CompletionRequest {
        CompletionRequest::new("echo", vec![crate::api::Message::user("hello")])
    }

    #[test]
    fn test_complete_echoes_last_user_text() {
        let completer = EchoCompleter;
        let request = sample_request();
        let resp = block_on(completer.complete(CompleteRequest { request: &request }))
            .expect("complete ok");
        assert_eq!(resp.content, Some("hello".to_string()));
    }

    #[test]
    fn test_model_info_unknown_model_errors() {
        let completer = EchoCompleter;
        let result = block_on(completer.model_info(ModelInfoRequest { model: "bogus" }));
        assert!(matches!(result, Err(CompleteError::ModelNotFound(_))));
    }

    #[test]
    fn test_validate_empty_model_errors() {
        let completer = EchoCompleter;
        let request = CompletionRequest::new("", vec![]);
        let result = completer.validate(ValidationRequest { request: &request });
        assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
    }
}
