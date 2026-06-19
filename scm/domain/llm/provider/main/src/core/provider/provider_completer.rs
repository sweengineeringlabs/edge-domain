//! `Completer` impl for `EchoProviderCompleter` — adapts this crate's `EchoExecutionModel`
//! to the `edge-llm-complete` port contract.

use async_trait::async_trait;
use edge_llm_complete::{
    CompleteError, Completer, CompletionRequest, CompletionResponse, CompletionStream, ContentPart,
    FinishReason, MessageContent, ModelInfo, Role, StreamChunk, StreamDelta, TokenUsage,
};
use futures::stream;

use crate::api::{
    EchoExecutionModel, EchoProviderCompleter, ExecutionConfig, ExecutionError, ExecutionModel,
    ExecutionMode,
};

impl EchoProviderCompleter {
    fn map_error(e: ExecutionError) -> CompleteError {
        match e {
            ExecutionError::ModelNotFound(m) => CompleteError::ModelNotFound(m),
            ExecutionError::AuthenticationFailed(m) => CompleteError::AuthenticationFailed(m),
            ExecutionError::RateLimited { retry_after_ms } => {
                CompleteError::RateLimited { retry_after_ms }
            }
            ExecutionError::ContextWindowExceeded { max_tokens, requested } => {
                CompleteError::ContextLengthExceeded { used: requested, max: max_tokens }
            }
            ExecutionError::Timeout { duration_ms } => CompleteError::Timeout(duration_ms),
            ExecutionError::InvalidRequest(m) => CompleteError::InvalidRequest(m),
            ExecutionError::StreamingError(m) => CompleteError::StreamError(m),
            ExecutionError::ContentFiltered(m) => CompleteError::ContentFiltered(m),
            ExecutionError::NetworkError(m) => CompleteError::NetworkError(m),
            ExecutionError::ProviderUnavailable { message } => {
                CompleteError::ProviderError { provider: "provider".to_string(), message }
            }
            ExecutionError::ValidationFailed(m) => CompleteError::InvalidRequest(m),
            ExecutionError::CacheError(m) => {
                CompleteError::InvalidRequest(format!("cache: {m}"))
            }
            ExecutionError::ToolCallFailed { tool_name, reason } => {
                CompleteError::InvalidRequest(format!("{tool_name}: {reason}"))
            }
            ExecutionError::QuotaExceeded { reset_at_ms } => {
                CompleteError::RateLimited { retry_after_ms: reset_at_ms }
            }
            ExecutionError::Unknown(m) => {
                CompleteError::ProviderError { provider: "provider".to_string(), message: m }
            }
        }
    }

    fn build_model() -> EchoExecutionModel {
        EchoExecutionModel::new(ExecutionConfig::new(4096, 30_000, false, true, ExecutionMode::Async))
    }

    fn extract_goal(request: &CompletionRequest) -> String {
        request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, Role::User))
            .map(|m| match &m.content {
                MessageContent::Text(t) => t.clone(),
                MessageContent::Parts(parts) => parts
                    .iter()
                    .filter_map(|p| {
                        if let ContentPart::Text { text } = p {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" "),
                MessageContent::Empty => String::new(),
            })
            .unwrap_or_default()
    }

    fn extract_context(request: &CompletionRequest) -> String {
        request
            .messages
            .iter()
            .map(|m| {
                let role = format!("{:?}", m.role).to_lowercase();
                let text = match &m.content {
                    MessageContent::Text(t) => t.clone(),
                    MessageContent::Parts(_) => "(multipart)".to_string(),
                    MessageContent::Empty => String::new(),
                };
                format!("{role}: {text}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[async_trait]
impl Completer for EchoProviderCompleter {
    async fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, CompleteError> {
        let model = Self::build_model();
        let goal = Self::extract_goal(request);
        let context = Self::extract_context(request);
        let tools: Vec<String> = request
            .tools
            .as_ref()
            .map(|t| t.iter().map(|td| td.name.clone()).collect())
            .unwrap_or_default();

        let result = model
            .execute_step("default", &goal, &context, tools)
            .await
            .map_err(Self::map_error)?;

        let usage = result
            .tokens_used
            .map(|u| TokenUsage::new(u.prompt_tokens, u.completion_tokens, u.total_tokens, 0, 0))
            .unwrap_or_else(|| TokenUsage::new(0, 0, 0, 0, 0));

        Ok(CompletionResponse {
            id: "provider-1".to_string(),
            model: request.model.clone(),
            content: Some(result.reasoning),
            finish_reason: FinishReason::Stop,
            usage: Box::new(usage),
            ..Default::default()
        })
    }

    async fn complete_stream(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionStream, CompleteError> {
        let response = self.complete(request).await?;
        let text = response.content.unwrap_or_default();
        let chunk = StreamChunk::terminal(
            "provider-stream-1",
            StreamDelta::text(text),
            FinishReason::Stop,
        );
        Ok(Box::pin(stream::once(async move { Ok::<StreamChunk, CompleteError>(chunk) })))
    }

    fn supported_models(&self) -> Vec<String> {
        vec![]
    }

    async fn model_info(&self, model: &str) -> Result<ModelInfo, CompleteError> {
        Err(CompleteError::ModelNotFound(model.to_string()))
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>, CompleteError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_llm_complete::{CompletionRequest, Message};
    use futures::executor::block_on;

    fn req(text: &str) -> CompletionRequest {
        CompletionRequest::new("echo", vec![Message::user(text)])
    }

    #[test]
    fn test_complete_happy_returns_response() {
        let result = block_on(EchoProviderCompleter.complete(&req("ping")));
        assert!(result.is_ok());
        let resp = result.expect("should succeed");
        assert!(resp.content.is_some());
        assert_eq!(resp.model, "echo");
    }

    #[test]
    fn test_complete_empty_messages_returns_response_error() {
        let empty_req = CompletionRequest::new("echo", vec![]);
        let result = block_on(EchoProviderCompleter.complete(&empty_req));
        assert!(result.is_ok());
    }

    #[test]
    fn test_complete_stream_happy_yields_chunk_edge() {
        let stream = block_on(EchoProviderCompleter.complete_stream(&req("stream test")));
        assert!(stream.is_ok());
    }

    #[test]
    fn test_supported_models_returns_empty_happy() {
        assert!(EchoProviderCompleter.supported_models().is_empty());
    }

    #[test]
    fn test_model_info_returns_not_found_error() {
        let result = block_on(EchoProviderCompleter.model_info("gpt-4"));
        assert!(result.is_err());
        assert!(matches!(result, Err(CompleteError::ModelNotFound(_))));
    }

    #[test]
    fn test_list_models_returns_empty_happy() {
        let result = block_on(EchoProviderCompleter.list_models());
        assert!(result.is_ok());
        assert!(result.expect("should succeed").is_empty());
    }
}
