#![allow(clippy::unwrap_used, clippy::expect_used)]
//! End-to-end coverage for `ConversationLoop` / `BoundedConversationLoop`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::DirectCommandBus;
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_agent::{
    Agent, AgentDescriptionRequest, AgentDescriptionResponse, AgentError, AgentIdRequest,
    AgentIdResponse, AgentManager, AgentNameRequest, AgentNameResponse, AgentProviderRequest,
    AgentProviderResponse, AgentSkillsRequest, AgentSkillsResponse, ConversationLoopRequest,
    ConversationRunRequest, Message, MessageContent, NoopAgentManager, OwnedHandlerContext,
    SkillExecutionRequest, SkillExecutionResponse,
};
use edge_llm_complete::{
    CompleteError, CompleteRequest, Completer, CompletionResponse, CompletionStreamRequest,
    CompletionStreamResponse, FinishReason, ListModelsRequest, ListModelsResponse,
    ModelInfoRequest, ModelInfoResponse as CompleteModelInfoResponse, SupportedModelsRequest,
    SupportedModelsResponse, ToolCall,
};
use edge_llm_provider::{ModelInfo, Provider, ProviderConfig, StdProvider};
use futures::executor::block_on;

struct ScriptedCompleter {
    responses: Vec<CompletionResponse>,
    calls: AtomicUsize,
}

#[async_trait]
impl Completer for ScriptedCompleter {
    async fn complete(
        &self,
        _req: CompleteRequest<'_>,
    ) -> Result<CompletionResponse, CompleteError> {
        let idx = self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(self
            .responses
            .get(idx)
            .or_else(|| self.responses.last())
            .expect("script must have at least one response")
            .clone())
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
    ) -> Result<CompleteModelInfoResponse, CompleteError> {
        Err(CompleteError::ModelNotFound("unused".to_string()))
    }
    async fn list_models(
        &self,
        _req: ListModelsRequest,
    ) -> Result<ListModelsResponse, CompleteError> {
        Ok(ListModelsResponse { models: vec![] })
    }
}

fn provider_with(responses: Vec<CompletionResponse>) -> Arc<dyn Provider> {
    Arc::new(StdProvider::new(
        ProviderConfig::new("test".to_string(), 0.0, 0),
        ModelInfo {
            id: "test-model".to_string(),
            ..Default::default()
        },
        Arc::new(ScriptedCompleter {
            responses,
            calls: AtomicUsize::new(0),
        }),
        StdObserveFactory::noop_arc_observe_context(),
    ))
}

fn stop_response(content: &str) -> CompletionResponse {
    CompletionResponse {
        content: Some(content.to_string()),
        finish_reason: FinishReason::Stop,
        ..Default::default()
    }
}

fn tool_call_response(calls: Vec<(&str, &str)>) -> CompletionResponse {
    CompletionResponse {
        tool_calls: calls
            .into_iter()
            .map(|(id, name)| ToolCall {
                id: id.to_string(),
                name: name.to_string(),
                arguments: "{}".to_string(),
            })
            .collect(),
        finish_reason: FinishReason::ToolCalls,
        ..Default::default()
    }
}

struct ScriptedAgent {
    provider: Arc<dyn Provider>,
    skill_calls: AtomicUsize,
    fail_skill: bool,
}

#[async_trait]
impl Agent for ScriptedAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: "scripted".to_string(),
        })
    }
    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: "scripted".to_string(),
        })
    }
    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: "scripted conversation loop test agent".to_string(),
        })
    }
    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        self.skill_calls.fetch_add(1, Ordering::SeqCst);
        if self.fail_skill {
            Err(AgentError::ExecutionFailed("skill boom".to_string()))
        } else {
            Ok(SkillExecutionResponse {
                output: format!("ran {}", req.skill_name),
            })
        }
    }
    fn skills(&self, _req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError> {
        Ok(AgentSkillsResponse { skills: vec![] })
    }
    fn provider(&self, _req: AgentProviderRequest) -> Result<AgentProviderResponse, AgentError> {
        Ok(AgentProviderResponse {
            provider: Arc::clone(&self.provider),
        })
    }
}

fn run_request(max_turns: u32) -> ConversationRunRequest {
    ConversationRunRequest {
        messages: vec![Message::user("hello")],
        max_turns,
        handler_context: Box::new(OwnedHandlerContext {
            security: SecurityContext::unauthenticated(),
            commands: Arc::new(DirectCommandBus),
            observer: StdObserveFactory::noop_observer_context(),
        }),
    }
}

/// @covers: ConversationLoop::run — happy: stops immediately when no tool calls requested
#[test]
fn test_run_single_turn_no_tool_calls_stops_immediately_happy() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![stop_response("done")]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: false,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(3))).unwrap();
    assert_eq!(result.turns, 1);
    assert_eq!(agent.skill_calls.load(Ordering::SeqCst), 0);
    assert_eq!(result.messages.len(), 2); // initial user message + assistant reply
}

/// @covers: ConversationLoop::run — happy: executes skill calls across turns then reaches stop
#[test]
fn test_run_executes_tool_calls_across_turns_and_reaches_stop_happy() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![
            tool_call_response(vec![("call-1", "get_weather")]),
            tool_call_response(vec![("call-2", "get_weather")]),
            stop_response("done"),
        ]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: false,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(5))).unwrap();
    assert_eq!(result.turns, 3);
    assert_eq!(agent.skill_calls.load(Ordering::SeqCst), 2);
    assert!(result
        .messages
        .iter()
        .any(|m| m.content == MessageContent::Text("ran get_weather".to_string())));
}

/// @covers: ConversationLoop::run — error: skill execution failure propagates as AgentError
#[test]
fn test_run_skill_execution_failure_propagates_as_agent_error_error() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![tool_call_response(vec![("call-1", "get_weather")])]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: true,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(3)));
    assert!(matches!(result, Err(AgentError::ExecutionFailed(msg)) if msg == "skill boom"));
}

/// @covers: ConversationLoop::run — error: hits turn limit without terminating
#[test]
fn test_run_hits_turn_limit_returns_turn_limit_exceeded_error() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![tool_call_response(vec![("call-1", "get_weather")])]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: false,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(2)));
    assert!(matches!(
        result,
        Err(AgentError::TurnLimitExceeded { max_turns: 2 })
    ));
}

/// @covers: ConversationLoop::run — edge: max_turns of zero is rejected
#[test]
fn test_run_max_turns_zero_returns_invalid_state_edge() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![stop_response("done")]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: false,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(0)));
    assert!(matches!(result, Err(AgentError::InvalidState(_))));
}

/// @covers: ConversationLoop::run — edge: multiple tool calls in one turn all execute, in order
#[test]
fn test_run_multiple_tool_calls_in_one_turn_all_execute_edge() {
    let agent = Arc::new(ScriptedAgent {
        provider: provider_with(vec![
            tool_call_response(vec![("call-1", "get_weather"), ("call-2", "get_time")]),
            stop_response("done"),
        ]),
        skill_calls: AtomicUsize::new(0),
        fail_skill: false,
    });
    let conversation_loop = NoopAgentManager
        .conversation_loop(ConversationLoopRequest {
            agent: Arc::clone(&agent) as Arc<dyn Agent>,
        })
        .unwrap()
        .conversation_loop;

    let result = block_on(conversation_loop.run(run_request(5))).unwrap();
    assert_eq!(agent.skill_calls.load(Ordering::SeqCst), 2);
    assert_eq!(result.turns, 2);

    let tool_messages: Vec<_> = result
        .messages
        .iter()
        .filter(|m| m.tool_call_id.is_some())
        .collect();
    assert_eq!(tool_messages.len(), 2);
    assert_eq!(tool_messages[0].tool_call_id, Some("call-1".to_string()));
    assert_eq!(
        tool_messages[0].content,
        MessageContent::Text("ran get_weather".to_string())
    );
    assert_eq!(tool_messages[1].tool_call_id, Some("call-2".to_string()));
    assert_eq!(
        tool_messages[1].content,
        MessageContent::Text("ran get_time".to_string())
    );
}
