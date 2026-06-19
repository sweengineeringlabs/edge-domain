//! `Completer`, `CompleteOps`, `ContentFlattener`, `ModelOps`, `StreamOps`, and `ToolOps`
//! impls for `NoopCompleter`.

use async_trait::async_trait;
use futures::stream;

use crate::api::{
    CompleteError, CompleteOps, CompletionRequest, CompletionResponse, CompletionStream,
    Completer, ContentFlattener, ContentPart, MessageContent, ModelInfo,
    ModelOps, NoopCompleter, StreamChunk, StreamDelta, StreamOps, ToolCall, ToolCallDelta,
    ToolChoice, ToolDefinition, ToolOps,
};

#[async_trait]
impl Completer for NoopCompleter {
    async fn complete(&self, _request: &CompletionRequest) -> Result<CompletionResponse, CompleteError> {
        Err(CompleteError::ProviderNotFound("noop".to_string()))
    }

    async fn complete_stream(
        &self,
        _request: &CompletionRequest,
    ) -> Result<CompletionStream, CompleteError> {
        Err(CompleteError::ProviderNotFound("noop".to_string()))
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

impl CompleteOps for NoopCompleter {
    fn check(&self, request: &CompletionRequest) -> Result<(), CompleteError> {
        if request.model.is_empty() {
            return Err(CompleteError::InvalidRequest("model cannot be empty".to_string()));
        }
        Ok(())
    }
}

impl ContentFlattener for NoopCompleter {
    fn flatten(&self, content: &MessageContent) -> String {
        match content {
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
        }
    }
}

#[async_trait]
impl ModelOps for NoopCompleter {
    async fn find_model(&self, name: &str) -> Result<ModelInfo, CompleteError> {
        Err(CompleteError::ModelNotFound(name.to_string()))
    }
}

impl StreamOps for NoopCompleter {
    fn apply_delta(
        &self,
        chunk: &mut StreamChunk,
        delta: &StreamDelta,
    ) -> Result<(), CompleteError> {
        *chunk.delta = delta.clone();
        Ok(())
    }
}

impl ToolOps for NoopCompleter {
    fn execute(&self, _call: &ToolCall) -> Result<String, CompleteError> {
        Err(CompleteError::InvalidRequest("no tools registered".to_string()))
    }

    fn available_tools(&self) -> Vec<ToolDefinition> {
        vec![]
    }

    fn tool_choice(&self) -> ToolChoice {
        ToolChoice::None
    }

    fn merge_delta(&self, existing: &mut ToolCallDelta, incoming: ToolCallDelta) {
        if incoming.id.is_some() {
            existing.id = incoming.id;
        }
        if incoming.name.is_some() {
            existing.name = incoming.name;
        }
        if incoming.arguments.is_some() {
            existing.arguments = incoming.arguments;
        }
    }
}

// Type-use anchor: keeps `stream` import needed for CompletionStream resolution.
const _: fn() = || {
    let _: CompletionStream = Box::pin(stream::empty::<Result<StreamChunk, CompleteError>>());
};
