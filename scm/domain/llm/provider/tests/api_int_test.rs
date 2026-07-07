//! Layer-level coverage for small api/ value types (Request/Response markers, errors, JsonValue)
//! that don't warrant their own dedicated test file — see SEA §5 Option C.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;

use edge_llm_provider::*;

#[test]
fn test_json_value_variants_construct() {
    let from_number: JsonValue = serde_json::json!(1).into();
    assert_eq!(from_number, JsonValue::Number(1.0));
    let from_null: JsonValue = serde_json::Value::Null.into();
    assert_eq!(from_null, JsonValue::Null);
    let from_bool: JsonValue = serde_json::json!(true).into();
    assert_eq!(from_bool, JsonValue::Bool(true));
}

#[test]
fn test_oauth_token_source_error_variants_construct() {
    let a = OauthTokenSourceError::CredentialFileUnreadable("x".to_string());
    let b = OauthTokenSourceError::MalformedCredentials("y".to_string());
    assert_ne!(a, b);
}

#[test]
fn test_accumulate_request_holds_delta() {
    let req = AccumulateRequest {
        delta: StreamDelta::empty(),
    };
    assert!(req.delta.is_empty());
}

#[test]
fn test_completer_request_response_construct() {
    let _req = CompleterRequest;
    let resp = CompleterResponse {
        completer: Arc::new(edge_llm_complete::NoopCompleter),
    };
    assert!(Arc::strong_count(&resp.completer) >= 1);
}

#[test]
fn test_execution_config_lookup_request_response_construct() {
    let _req = ExecutionConfigLookupRequest;
    let resp = ExecutionConfigResponse {
        config: Box::new(ExecutionConfig::new(
            1,
            1,
            false,
            false,
            ExecutionMode::Async,
        )),
    };
    assert_eq!(resp.config.max_tokens_per_call, 1);
}

#[test]
fn test_execution_mode_lookup_request_response_construct() {
    let _req = ExecutionModeLookupRequest;
    let resp = ExecutionModeResponse {
        mode: ExecutionMode::Streaming,
    };
    assert_eq!(resp.mode, ExecutionMode::Streaming);
}

#[test]
fn test_execution_readiness_request_constructs() {
    let req = ExecutionReadinessRequest;
    assert_eq!(format!("{req:?}"), "ExecutionReadinessRequest");
}

#[test]
fn test_health_check_request_constructs() {
    let req = HealthCheckRequest;
    assert_eq!(format!("{req:?}"), "HealthCheckRequest");
}

#[test]
fn test_last_finish_reason_request_response_construct() {
    let _req = LastFinishReasonRequest;
    let resp = LastFinishReasonResponse {
        reason: FinishReason::Stop,
    };
    assert_eq!(resp.reason, FinishReason::Stop);
}

#[test]
fn test_last_token_usage_request_response_construct() {
    let _req = LastTokenUsageRequest;
    let resp = LastTokenUsageResponse {
        usage: Box::new(TokenUsage::new(1, 2, 0, 0)),
    };
    assert_eq!(resp.usage.total_tokens, 3);
}

#[test]
fn test_model_family_request_response_construct() {
    let _req = ModelFamilyRequest;
    let resp = ModelFamilyResponse {
        family: ModelFamily::OpenAI,
    };
    assert_eq!(resp.family, ModelFamily::OpenAI);
}

#[test]
fn test_model_info_lookup_request_response_construct() {
    let _req = ModelInfoLookupRequest;
    let resp = ModelInfoResponse {
        info: Box::new(ModelInfo::new(
            "id".to_string(),
            "name".to_string(),
            ModelFamily::Google,
            1000,
        )),
    };
    assert_eq!(resp.info.id, "id");
}

#[test]
fn test_next_chunk_request_response_construct() {
    let _req = NextChunkRequest;
    let resp = NextChunkResponse { chunk: None };
    assert!(resp.chunk.is_none());
}

#[test]
fn test_pending_tool_call_request_response_construct() {
    let _req = PendingToolCallRequest;
    let resp = PendingToolCallResponse { tool_call: None };
    assert!(resp.tool_call.is_none());
}

#[test]
fn test_provider_config_lookup_request_response_construct() {
    let _req = ProviderConfigLookupRequest;
    let resp = ProviderConfigResponse {
        config: Box::new(ProviderConfig::new("m".to_string(), 0.5, 1000)),
    };
    assert_eq!(resp.config.model, "m");
}

#[test]
fn test_provider_name_request_response_construct() {
    let _req = ProviderNameRequest;
    let resp = ProviderNameResponse {
        name: "claude".to_string(),
    };
    assert_eq!(resp.name, "claude");
}

#[test]
fn test_step_execution_request_response_construct() {
    let req = StepExecutionRequest {
        agent_id: "a1",
        goal: "ship",
        context: "ctx",
        available_tools: vec![],
    };
    assert_eq!(req.goal, "ship");
    let resp = StepExecutionResponse {
        result: Box::new(ExecutionStepResult::new(
            "reasoning".to_string(),
            None,
            0.5,
            None,
        )),
    };
    assert_eq!(resp.result.reasoning, "reasoning");
}

#[test]
fn test_tokenizer_accuracy_request_response_construct() {
    let _req = TokenizerAccuracyRequest;
    let resp = TokenizerAccuracyResponse {
        accuracy: TokenizerAccuracy::Exact,
    };
    assert_eq!(resp.accuracy, TokenizerAccuracy::Exact);
}

#[test]
fn test_token_source_file_request_response_construct() {
    let req = TokenSourceFileRequest {
        path: Path::new("/tmp/x"),
    };
    assert_eq!(req.path, Path::new("/tmp/x"));
    let resp = TokenSourceInitResponse {
        source: Arc::new(1u32),
    };
    assert_eq!(*resp.source.downcast_ref::<u32>().expect("u32"), 1);
}
