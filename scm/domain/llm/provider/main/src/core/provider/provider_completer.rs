//! `Completer` impl for `EchoProviderCompleter` — adapts this crate's `EchoExecutionModel`
//! to the `edge-llm-complete` port contract.

use async_trait::async_trait;
use edge_llm_complete::{
    CompleteError, CompleteRequest, Completer, CompletionRequest, CompletionResponse,
    CompletionStreamRequest, CompletionStreamResponse, ContentPart, FinishReason, ListModelsRequest,
    ListModelsResponse, MessageContent, ModelInfoRequest, ModelInfoResponse, Role, StreamChunk,
    StreamDelta, SupportedModelsRequest, SupportedModelsResponse, TokenUsage,
};
use futures::stream;

use crate::api::{
    EchoExecutionModel, EchoProviderCompleter, ExecutionConfig, ExecutionError, ExecutionMode,
    ExecutionModel, StepExecutionRequest,
};

impl EchoProviderCompleter {
    fn map_error(e: ExecutionError) -> CompleteError {
        match e {
            ExecutionError::ModelNotFound(m) => CompleteError::ModelNotFound(m),
            ExecutionError::AuthenticationFailed(m) => CompleteError::AuthenticationFailed(m),
            ExecutionError::RateLimited { retry_after_ms } => {
                CompleteError::RateLimited { retry_after_ms }
            }
            ExecutionError::ContextWindowExceeded {
                max_tokens,
                requested,
            } => CompleteError::ContextLengthExceeded {
                used: requested,
                max: max_tokens,
            },
            ExecutionError::Timeout { duration_ms } => CompleteError::Timeout(duration_ms),
            ExecutionError::InvalidRequest(m) => CompleteError::InvalidRequest(m),
            ExecutionError::StreamingError(m) => CompleteError::StreamError(m),
            ExecutionError::ContentFiltered(m) => CompleteError::ContentFiltered(m),
            ExecutionError::NetworkError(m) => CompleteError::NetworkError(m),
            ExecutionError::ProviderUnavailable { message } => CompleteError::ProviderError {
                provider: "provider".to_string(),
                message,
            },
            ExecutionError::ValidationFailed(m) => CompleteError::InvalidRequest(m),
            ExecutionError::CacheError(m) => CompleteError::InvalidRequest(format!("cache: {m}")),
            ExecutionError::ToolCallFailed { tool_name, reason } => {
                CompleteError::InvalidRequest(format!("{tool_name}: {reason}"))
            }
            ExecutionError::QuotaExceeded { reset_at_ms } => CompleteError::RateLimited {
                retry_after_ms: reset_at_ms,
            },
            ExecutionError::Unknown(m) => CompleteError::ProviderError {
                provider: "provider".to_string(),
                message: m,
            },
        }
    }

    fn build_model() -> EchoExecutionModel {
        EchoExecutionModel::new(ExecutionConfig::new(
            4096,
            30_000,
            false,
            true,
            ExecutionMode::Async,
        ))
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
        req: CompleteRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        let request = req.request;
        let model = Self::build_model();
        let goal = Self::extract_goal(request);
        let context = Self::extract_context(request);
        let tools: Vec<String> = request
            .tools
            .as_ref()
            .map(|t| t.iter().map(|td| td.name.clone()).collect())
            .unwrap_or_default();

        let result = model
            .execute_step(StepExecutionRequest {
                agent_id: "default",
                goal: &goal,
                context: &context,
                available_tools: tools,
            })
            .await
            .map_err(Self::map_error)?
            .result;

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
        req: CompletionStreamRequest<'_>,
    ) -> Result<CompletionStreamResponse, CompleteError> {
        let response = self.complete(CompleteRequest { request: req.request }).await?;
        let text = response.content.unwrap_or_default();
        let chunk = StreamChunk::terminal(
            "provider-stream-1",
            StreamDelta::text(text),
            FinishReason::Stop,
        );
        Ok(CompletionStreamResponse {
            stream: Box::pin(stream::once(async move {
                Ok::<StreamChunk, CompleteError>(chunk)
            })),
        })
    }

    fn supported_models(
        &self,
        _req: SupportedModelsRequest,
    ) -> Result<SupportedModelsResponse, CompleteError> {
        Ok(SupportedModelsResponse { models: vec![] })
    }

    async fn model_info(
        &self,
        req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError> {
        Err(CompleteError::ModelNotFound(req.model.to_string()))
    }

    async fn list_models(
        &self,
        _req: ListModelsRequest,
    ) -> Result<ListModelsResponse, CompleteError> {
        Ok(ListModelsResponse { models: vec![] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_llm_complete::{CompletionRequest, Message};
    use futures::executor::block_on;
    use futures::StreamExt;

    fn req(text: &str) -> CompletionRequest {
        CompletionRequest::new("echo", vec![Message::user(text)])
    }

    #[test]
    fn test_complete_happy_returns_response() {
        let request = req("ping");
        let result = block_on(EchoProviderCompleter.complete(CompleteRequest { request: &request }));
        assert!(result.is_ok());
        let resp = result.expect("should succeed");
        assert!(resp.content.is_some());
        assert_eq!(resp.model, "echo");
    }

    #[test]
    fn test_complete_empty_messages_returns_response_error() {
        let empty_req = CompletionRequest::new("echo", vec![]);
        let result = block_on(EchoProviderCompleter.complete(CompleteRequest {
            request: &empty_req,
        }))
        .expect("should succeed");
        assert_eq!(result.finish_reason, FinishReason::Stop);
    }

    #[test]
    fn test_complete_stream_happy_yields_chunk_edge() {
        let request = req("stream test");
        let response = block_on(
            EchoProviderCompleter.complete_stream(CompletionStreamRequest { request: &request }),
        )
        .expect("should succeed");
        let mut stream = response.stream;
        let chunk = block_on(stream.next())
            .expect("one chunk")
            .expect("ok chunk");
        assert_eq!(chunk.finish_reason, Some(FinishReason::Stop));
    }

    #[test]
    fn test_supported_models_returns_empty_happy() {
        let result = EchoProviderCompleter
            .supported_models(SupportedModelsRequest)
            .expect("should succeed");
        assert!(result.models.is_empty());
    }

    #[test]
    fn test_model_info_returns_not_found_error() {
        let result = block_on(EchoProviderCompleter.model_info(ModelInfoRequest { model: "gpt-4" }));
        assert!(result.is_err());
        assert!(matches!(result, Err(CompleteError::ModelNotFound(_))));
    }

    #[test]
    fn test_list_models_returns_empty_happy() {
        let result = block_on(EchoProviderCompleter.list_models(ListModelsRequest));
        let response = result.expect("should succeed");
        assert!(response.models.is_empty());
    }

    /// @covers: map_error
    #[test]
    fn test_map_error_model_not_found_maps_directly_happy() {
        let mapped = EchoProviderCompleter::map_error(ExecutionError::ModelNotFound(
            "gpt-9".to_string(),
        ));
        assert!(matches!(mapped, CompleteError::ModelNotFound(m) if m == "gpt-9"));
    }

    /// @covers: map_error
    #[test]
    fn test_map_error_context_window_exceeded_swaps_field_names_edge() {
        // ExecutionError uses {max_tokens, requested}; CompleteError uses {used, max} —
        // verify the fields land in the correct (swapped-name) slots, not just that
        // some ContextLengthExceeded value comes out.
        let mapped = EchoProviderCompleter::map_error(ExecutionError::ContextWindowExceeded {
            max_tokens: 100,
            requested: 150,
        });
        assert!(matches!(
            mapped,
            CompleteError::ContextLengthExceeded { used: 150, max: 100 }
        ));
    }

    /// @covers: map_error
    #[test]
    fn test_map_error_quota_exceeded_maps_to_rate_limited_error() {
        let mapped = EchoProviderCompleter::map_error(ExecutionError::QuotaExceeded {
            reset_at_ms: Some(5000),
        });
        assert!(matches!(
            mapped,
            CompleteError::RateLimited { retry_after_ms: Some(5000) }
        ));
    }

    /// @covers: build_model
    #[test]
    fn test_build_model_constructs_async_execution_model_happy() {
        let model = EchoProviderCompleter::build_model();
        let response = block_on(model.execute_step(StepExecutionRequest {
            agent_id: "default",
            goal: "test goal",
            context: "",
            available_tools: vec![],
        }))
        .expect("execute_step should succeed");
        assert!(
            response.result.reasoning.contains("test goal"),
            "expected the goal to be echoed back into the reasoning, got: {}",
            response.result.reasoning
        );
    }

    /// @covers: build_model
    #[test]
    fn test_build_model_different_goals_produce_different_reasoning_edge() {
        let model = EchoProviderCompleter::build_model();
        let a = block_on(model.execute_step(StepExecutionRequest {
            agent_id: "default",
            goal: "goal A",
            context: "",
            available_tools: vec![],
        }))
        .expect("execute_step should succeed");
        let b = block_on(model.execute_step(StepExecutionRequest {
            agent_id: "default",
            goal: "goal B",
            context: "",
            available_tools: vec![],
        }))
        .expect("execute_step should succeed");
        assert_ne!(a.result.reasoning, b.result.reasoning);
    }

    /// @covers: extract_goal
    #[test]
    fn test_extract_goal_returns_last_user_message_happy() {
        let request = CompletionRequest::new(
            "echo",
            vec![Message::user("first"), Message::user("second")],
        );
        assert_eq!(EchoProviderCompleter::extract_goal(&request), "second");
    }

    /// @covers: extract_goal
    #[test]
    fn test_extract_goal_no_user_message_returns_empty_edge() {
        let request = CompletionRequest::new("echo", vec![Message::system("sys prompt")]);
        assert_eq!(EchoProviderCompleter::extract_goal(&request), "");
    }

    /// @covers: extract_context
    #[test]
    fn test_extract_context_joins_all_messages_with_role_prefix_happy() {
        let request = CompletionRequest::new(
            "echo",
            vec![Message::system("be nice"), Message::user("hello")],
        );
        let context = EchoProviderCompleter::extract_context(&request);
        assert!(context.contains("system: be nice"));
        assert!(context.contains("user: hello"));
    }

    /// @covers: extract_context
    #[test]
    fn test_extract_context_empty_messages_returns_empty_string_edge() {
        let request = CompletionRequest::new("echo", vec![]);
        assert_eq!(EchoProviderCompleter::extract_context(&request), "");
    }
}
