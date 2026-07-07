//! Layer-level coverage for small api/ Request/Response marker types that don't
//! warrant their own dedicated test file — see SEA §5 Option C.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::*;
use serde_json::json;

#[test]
fn test_available_tools_request_response_construct() {
    let _req = AvailableToolsRequest;
    let resp = AvailableToolsResponse {
        tools: vec![ToolDefinition::new("search", "search the web", json!({}))],
    };
    assert_eq!(resp.tools.len(), 1);
}

#[test]
fn test_cache_control_request_response_construct() {
    let req = CacheControlRequest {
        cache: Box::new(CacheControl::ephemeral()),
    };
    assert_eq!(req.cache.cache_type, "ephemeral");
    let resp = CacheControlResponse {
        message: Message::user("hi"),
    };
    assert_eq!(resp.message.role, Role::User);
}

#[test]
fn test_completer_health_check_request_response_construct() {
    let _req = CompleterHealthCheckRequest;
    let resp = CompleterHealthCheckResponse { healthy: true };
    assert!(resp.healthy);
}

#[test]
fn test_complete_request_constructs() {
    let completion = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let req = CompleteRequest {
        request: &completion,
    };
    assert_eq!(req.request.model, "echo");
}

#[test]
fn test_completion_check_request_constructs() {
    let completion = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let req = CompletionCheckRequest {
        request: &completion,
    };
    assert_eq!(req.request.model, "echo");
}

#[test]
fn test_completion_stream_request_response_construct() {
    let completion = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let req = CompletionStreamRequest {
        request: &completion,
    };
    assert_eq!(req.request.model, "echo");
    let resp = CompletionStreamResponse {
        stream: Box::pin(futures::stream::empty()),
    };
    let _ = resp.stream;
}

#[test]
fn test_delta_application_request_constructs() {
    let mut chunk = StreamChunk::partial("c-1", StreamDelta::empty());
    let delta = StreamDelta::text("hi");
    let req = DeltaApplicationRequest {
        chunk: &mut chunk,
        delta: &delta,
    };
    assert_eq!(req.delta.content, Some("hi".to_string()));
}

#[test]
fn test_delta_merge_request_constructs() {
    let mut existing = ToolCallDelta::new(0);
    let req = DeltaMergeRequest {
        existing: &mut existing,
        incoming: Box::new(ToolCallDelta::new(0).with_name("search")),
    };
    assert_eq!(req.incoming.name, Some("search".to_string()));
}

#[test]
fn test_flatten_request_response_construct() {
    let content = MessageContent::Text("hi".to_string());
    let req = FlattenRequest { content: &content };
    assert!(matches!(req.content, MessageContent::Text(_)));
    let resp = FlattenResponse {
        text: "hi".to_string(),
    };
    assert_eq!(resp.text, "hi");
}

#[test]
fn test_list_models_request_response_construct() {
    let _req = ListModelsRequest;
    let resp = ListModelsResponse {
        models: vec![ModelInfo::new("m", "M", "p", 0)],
    };
    assert_eq!(resp.models.len(), 1);
}

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_mark_ephemeral_request_constructs() {
    let req = MarkEphemeralRequest;
    assert_eq!(req, MarkEphemeralRequest::default());
}

#[test]
fn test_model_availability_request_response_construct() {
    let req = ModelAvailabilityRequest { model: "echo" };
    assert_eq!(req.model, "echo");
    let resp = ModelAvailabilityResponse { available: true };
    assert!(resp.available);
}

#[test]
fn test_model_info_request_response_construct() {
    let req = ModelInfoRequest { model: "echo" };
    assert_eq!(req.model, "echo");
    let resp = ModelInfoResponse {
        info: Box::new(ModelInfo::new("echo", "Echo", "echo", 0)),
    };
    assert_eq!(resp.info.id, "echo");
}

#[test]
fn test_model_support_request_response_construct() {
    let req = ModelSupportRequest { model: "echo" };
    assert_eq!(req.model, "echo");
    let resp = ModelSupportResponse { supported: true };
    assert!(resp.supported);
}

#[test]
fn test_processing_request_constructs() {
    let completion = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let req = ProcessingRequest {
        request: &completion,
    };
    assert_eq!(req.request.model, "echo");
}

#[test]
fn test_supported_models_request_response_construct() {
    let _req = SupportedModelsRequest;
    let resp = SupportedModelsResponse {
        models: vec!["echo".to_string()],
    };
    assert_eq!(resp.models, vec!["echo".to_string()]);
}

#[test]
fn test_tool_choice_preference_request_response_construct() {
    let _req = ToolChoicePreferenceRequest;
    let resp = ToolChoicePreferenceResponse {
        choice: ToolChoice::Auto,
    };
    assert_eq!(resp.choice, ToolChoice::Auto);
}

#[test]
fn test_tool_execution_request_response_construct() {
    let call = ToolCall::new("id-1", "search", "{}");
    let req = ToolExecutionRequest { call: &call };
    assert_eq!(req.call.name, "search");
    let resp = ToolExecutionResponse {
        output: "ok".to_string(),
    };
    assert_eq!(resp.output, "ok");
}

#[test]
fn test_validation_request_constructs() {
    let completion = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let req = ValidationRequest {
        request: &completion,
    };
    assert_eq!(req.request.model, "echo");
}
