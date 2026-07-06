//! `Completer`, `CompleteOps`, `ContentFlattener`, `ModelOps`, `StreamOps`, and `ToolOps`
//! impls for `NoopCompleter`.

use async_trait::async_trait;
use futures::stream;

use crate::api::{
    AvailableToolsRequest, AvailableToolsResponse, CompleteError, CompleteOps, CompleteRequest,
    Completer, CompletionCheckRequest, CompletionRequest, CompletionResponse,
    CompletionStreamRequest, CompletionStreamResponse, ContentFlattener, ContentPart,
    DeltaApplicationRequest, DeltaMergeRequest, FlattenRequest, FlattenResponse, ListModelsRequest,
    ListModelsResponse, MessageContent, ModelInfo, ModelInfoRequest, ModelInfoResponse, ModelOps,
    NoopCompleter, StreamChunk, StreamOps, SupportedModelsRequest, SupportedModelsResponse,
    ToolChoice, ToolChoicePreferenceRequest, ToolChoicePreferenceResponse, ToolExecutionRequest,
    ToolExecutionResponse, ToolOps,
};

#[async_trait]
impl Completer for NoopCompleter {
    async fn complete(
        &self,
        _req: CompleteRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        Err(CompleteError::ProviderNotFound("noop".to_string()))
    }

    async fn complete_stream(
        &self,
        _req: CompletionStreamRequest<'_>,
    ) -> Result<CompletionStreamResponse, CompleteError> {
        Err(CompleteError::ProviderNotFound("noop".to_string()))
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

impl CompleteOps for NoopCompleter {
    fn check(&self, req: CompletionCheckRequest<'_>) -> Result<(), CompleteError> {
        if req.request.model.is_empty() {
            return Err(CompleteError::InvalidRequest(
                "model cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

impl ContentFlattener for NoopCompleter {
    fn flatten(&self, req: FlattenRequest<'_>) -> Result<FlattenResponse, CompleteError> {
        let text = match req.content {
            MessageContent::Empty => String::new(),
            MessageContent::Text(t) => t.clone(),
            MessageContent::Parts(parts) => parts
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        };
        Ok(FlattenResponse { text })
    }
}

#[async_trait]
impl ModelOps for NoopCompleter {
    async fn find_model(
        &self,
        req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError> {
        Err(CompleteError::ModelNotFound(req.model.to_string()))
    }
}

impl StreamOps for NoopCompleter {
    fn apply_delta(&self, req: DeltaApplicationRequest<'_>) -> Result<(), CompleteError> {
        *req.chunk.delta = req.delta.clone();
        Ok(())
    }
}

impl ToolOps for NoopCompleter {
    fn execute(
        &self,
        _req: ToolExecutionRequest<'_>,
    ) -> Result<ToolExecutionResponse, CompleteError> {
        Err(CompleteError::InvalidRequest(
            "no tools registered".to_string(),
        ))
    }

    fn available_tools(
        &self,
        _req: AvailableToolsRequest,
    ) -> Result<AvailableToolsResponse, CompleteError> {
        Ok(AvailableToolsResponse { tools: vec![] })
    }

    fn tool_choice(
        &self,
        _req: ToolChoicePreferenceRequest,
    ) -> Result<ToolChoicePreferenceResponse, CompleteError> {
        Ok(ToolChoicePreferenceResponse {
            choice: ToolChoice::None,
        })
    }

    fn merge_delta(&self, req: DeltaMergeRequest<'_>) -> Result<(), CompleteError> {
        if req.incoming.id.is_some() {
            req.existing.id = req.incoming.id;
        }
        if req.incoming.name.is_some() {
            req.existing.name = req.incoming.name;
        }
        if req.incoming.arguments.is_some() {
            req.existing.arguments = req.incoming.arguments;
        }
        Ok(())
    }
}

// Type-use anchor: keeps the `stream`/`ModelInfo` imports needed for resolution.
const _: fn() = || {
    let _: crate::api::CompletionStream =
        Box::pin(stream::empty::<Result<StreamChunk, CompleteError>>());
    let _ = ModelInfo::default();
};

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn test_complete_returns_provider_not_found() {
        let completer = NoopCompleter;
        let request = CompletionRequest::default();
        let result = block_on(completer.complete(CompleteRequest { request: &request }));
        assert!(matches!(result, Err(CompleteError::ProviderNotFound(_))));
    }

    #[test]
    fn test_check_empty_model_errors() {
        let completer = NoopCompleter;
        let request = CompletionRequest::default();
        let result = completer.check(CompletionCheckRequest { request: &request });
        assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
    }

    #[test]
    fn test_flatten_text_returns_text() {
        let completer = NoopCompleter;
        let content = MessageContent::Text("hi".to_string());
        let resp = completer
            .flatten(FlattenRequest { content: &content })
            .expect("flatten ok");
        assert_eq!(resp.text, "hi");
    }

    #[test]
    fn test_merge_delta_overwrites_present_fields() {
        let completer = NoopCompleter;
        let mut existing = crate::api::ToolCallDelta::new(0);
        let incoming = crate::api::ToolCallDelta::new(0).with_name("get_weather");
        completer
            .merge_delta(DeltaMergeRequest {
                existing: &mut existing,
                incoming: Box::new(incoming),
            })
            .expect("merge_delta ok");
        assert_eq!(existing.name, Some("get_weather".to_string()));
    }
}
