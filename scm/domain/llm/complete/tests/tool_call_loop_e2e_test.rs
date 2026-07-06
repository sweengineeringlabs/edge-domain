//! Scenario coverage for the `ToolCallLoop` trait and its `CompleteBootstrap` factory.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use edge_llm_complete::{
    AvailableToolsRequest, AvailableToolsResponse, CompleteBootstrap, CompleteError,
    CompleteRequest, Completer, CompletionRequest, CompletionResponse, CompletionStreamRequest,
    CompletionStreamResponse, DeltaMergeRequest, FinishReason, ListModelsRequest,
    ListModelsResponse, ModelInfoRequest, ModelInfoResponse, Role, SupportedModelsRequest,
    SupportedModelsResponse, TokenUsage, ToolCall, ToolCallLoopRequest,
    ToolChoicePreferenceRequest, ToolChoicePreferenceResponse, ToolExecutionRequest,
    ToolExecutionResponse, ToolOps,
};

struct ScriptedCompleter {
    tool_turns: usize,
    calls: AtomicUsize,
}

#[async_trait]
impl Completer for ScriptedCompleter {
    async fn complete(
        &self,
        _req: CompleteRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        let call_index = self.calls.fetch_add(1, Ordering::SeqCst);
        if call_index < self.tool_turns {
            Ok(CompletionResponse {
                id: format!("turn-{call_index}"),
                model: "test-model".to_string(),
                content: None,
                tool_calls: vec![ToolCall::new(
                    format!("call-{call_index}"),
                    "get_weather".to_string(),
                    "{}".to_string(),
                )],
                finish_reason: FinishReason::ToolCalls,
                usage: Box::new(TokenUsage::default()),
            })
        } else {
            Ok(CompletionResponse {
                id: "final".to_string(),
                model: "test-model".to_string(),
                content: Some("done".to_string()),
                tool_calls: vec![],
                finish_reason: FinishReason::Stop,
                usage: Box::new(TokenUsage::default()),
            })
        }
    }

    async fn complete_stream(
        &self,
        _req: CompletionStreamRequest<'_>,
    ) -> Result<CompletionStreamResponse, CompleteError> {
        Err(CompleteError::InvalidRequest(
            "not used in tests".to_string(),
        ))
    }

    fn supported_models(
        &self,
        _req: SupportedModelsRequest,
    ) -> Result<SupportedModelsResponse, CompleteError> {
        Ok(SupportedModelsResponse { models: vec![] })
    }

    async fn model_info(
        &self,
        _req: ModelInfoRequest<'_>,
    ) -> Result<ModelInfoResponse, CompleteError> {
        Err(CompleteError::ModelNotFound("unused".to_string()))
    }

    async fn list_models(
        &self,
        _req: ListModelsRequest,
    ) -> Result<ListModelsResponse, CompleteError> {
        Ok(ListModelsResponse { models: vec![] })
    }
}

struct FailingToolOps;
impl ToolOps for FailingToolOps {
    fn execute(
        &self,
        _req: ToolExecutionRequest<'_>,
    ) -> Result<ToolExecutionResponse, CompleteError> {
        Err(CompleteError::InvalidRequest("tool failed".to_string()))
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
            choice: edge_llm_complete::ToolChoice::Auto,
        })
    }
    fn merge_delta(&self, _req: DeltaMergeRequest<'_>) -> Result<(), CompleteError> {
        Ok(())
    }
}

struct EchoToolOps;
impl ToolOps for EchoToolOps {
    fn execute(
        &self,
        req: ToolExecutionRequest<'_>,
    ) -> Result<ToolExecutionResponse, CompleteError> {
        Ok(ToolExecutionResponse {
            output: format!("ran {}", req.call.name),
        })
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
            choice: edge_llm_complete::ToolChoice::Auto,
        })
    }
    fn merge_delta(&self, _req: DeltaMergeRequest<'_>) -> Result<(), CompleteError> {
        Ok(())
    }
}

struct TestFactory;
impl CompleteBootstrap for TestFactory {}

// ── ToolCallLoop::run ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_run_single_turn_completion_returns_immediately_happy() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 0,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 3,
        })
        .await
        .expect("run ok");
    assert_eq!(result.turns, 1);
    assert_eq!(result.response.finish_reason, FinishReason::Stop);
}

#[tokio::test]
async fn test_run_executes_multiple_tool_calls_concurrently_and_reaches_stop_happy() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 2,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 5,
        })
        .await
        .expect("run ok");
    assert_eq!(result.turns, 3);
    assert!(result.messages.iter().any(|m| m.role == Role::Tool));
}

#[tokio::test]
async fn test_run_tool_execution_failure_propagates_as_complete_error_error() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 1,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(FailingToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 3,
        })
        .await;
    assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
}

#[tokio::test]
async fn test_run_hits_turn_limit_returns_turn_limit_exceeded_error() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 10,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 2,
        })
        .await;
    assert!(matches!(
        result,
        Err(CompleteError::TurnLimitExceeded { max_turns: 2 })
    ));
}

// ── CompleteBootstrap::tool_call_loop ─────────────────────────────────────────

#[tokio::test]
async fn test_tool_call_loop_factory_builds_working_instance_happy() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 0,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    assert!(l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 1,
        })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_tool_call_loop_factory_wires_failing_tool_ops_error() {
    let l = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 1,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(FailingToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 3,
        })
        .await;
    assert!(matches!(result, Err(CompleteError::InvalidRequest(_))));
}

#[tokio::test]
async fn test_tool_call_loop_factory_builds_independent_instances_edge() {
    let l1 = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 0,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let l2 = TestFactory::tool_call_loop(
        Arc::new(ScriptedCompleter {
            tool_turns: 0,
            calls: AtomicUsize::new(0),
        }),
        Arc::new(EchoToolOps),
    );
    let request = CompletionRequest::new("test-model", vec![]);
    let r1 = l1
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 1,
        })
        .await
        .expect("run ok");
    let r2 = l2
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 1,
        })
        .await
        .expect("run ok");
    assert_eq!(r1.turns, r2.turns);
}
