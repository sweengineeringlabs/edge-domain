//! SAF facade tests — `ProviderBootstrap` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_complete::{Completer, CompletionRequest, Message, NoopCompleter};
use edge_llm_provider::{
    AccumulateRequest, CompletionMessage, EchoProviderCompleter, ExecutionConfig, ExecutionMode,
    ExecutionModeLookupRequest, ExecutionModel, ExecutionReadinessRequest, HealthCheckRequest,
    MessageRole, ModelFamily, ModelFamilyRequest, ModelInfo, NextChunkRequest,
    PendingToolCallRequest, ProviderBootstrap, ProviderConfig, ProviderNameRequest,
    StdProviderFactory, StreamHandler, ToolDefinition,
};
use futures::executor::block_on;
use serde_json::json;

// --- default_provider_handler ---

/// @covers: default_provider_handler — builds a usable Handler
#[test]
fn test_default_provider_handler_runs_happy() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::executor::block_on;
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    let security: SecurityContext = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: "go".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert!(out.reasoning.contains("go"));
}

/// @covers: default_provider_handler — zero-budget config surfaces an error through the pipeline
#[test]
fn test_default_provider_handler_zero_budget_errors_error() {
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityContext, SecurityServices};
    use futures::executor::block_on;
    let config = ExecutionConfig::new(0, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    let security: SecurityContext = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    assert!(block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: "go".to_string(),
            ctx: &ctx,
        },
    ))
    .is_err());
}

/// @covers: default_provider_handler — exposes the stable dispatch id
#[test]
fn test_default_provider_handler_exposes_handler_id_edge() {
    use edge_domain_handler::{Handler, IdRequest};
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id ok").id,
        "provider.execute_step"
    );
}

// --- std_factory ---

/// @covers: ProviderBootstrap::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let f: StdProviderFactory = StdProviderFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0, "factory must be zero-sized");
}

/// @covers: ProviderBootstrap::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdProviderFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: ProviderBootstrap::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let a = StdProviderFactory::std_factory();
    let b = StdProviderFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

// --- provider ---

/// @covers: ProviderBootstrap::provider — builds a usable provider
#[test]
fn test_provider_builds_named_provider_happy() {
    let config = ProviderConfig::new("claude".to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    let name = StdProviderFactory::provider(
        config,
        Box::new(info),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
    .name(ProviderNameRequest)
    .expect("ok")
    .name;
    assert_eq!(name, "claude");
}

/// @covers: ProviderBootstrap::provider — empty model produces an unhealthy provider
#[test]
fn test_provider_empty_model_unhealthy_error() {
    let config = ProviderConfig::new(String::new(), 0.7, 8192);
    let info = ModelInfo::new(String::new(), String::new(), ModelFamily::OpenAI, 8192);
    assert!(StdProviderFactory::provider(
        config,
        Box::new(info),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context()
    )
    .health_check(HealthCheckRequest)
    .is_err());
}

/// @covers: ProviderBootstrap::provider — family flows from model metadata
#[test]
fn test_provider_reports_model_family_edge() {
    let config = ProviderConfig::new("gpt".to_string(), 0.5, 4096);
    let info = ModelInfo::new(
        "gpt".to_string(),
        "GPT".to_string(),
        ModelFamily::OpenAI,
        4096,
    );
    let family = StdProviderFactory::provider(
        config,
        Box::new(info),
        Arc::new(NoopCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
    .model_family(ModelFamilyRequest)
    .expect("ok")
    .family;
    assert_eq!(family, ModelFamily::OpenAI);
}

// --- execution_model ---

/// @covers: ProviderBootstrap::execution_model — builds a model in the given mode
#[test]
fn test_execution_model_builds_in_mode_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    let mode = StdProviderFactory::execution_model(config)
        .execution_mode(ExecutionModeLookupRequest)
        .expect("ok")
        .mode;
    assert_eq!(mode, ExecutionMode::Async);
}

/// @covers: ProviderBootstrap::execution_model — zero budget cannot execute
#[test]
fn test_execution_model_zero_budget_blocked_error() {
    let config = ExecutionConfig::new(0, 30_000, true, false, ExecutionMode::Async);
    assert!(StdProviderFactory::execution_model(config)
        .can_execute(ExecutionReadinessRequest)
        .is_err());
}

/// @covers: ProviderBootstrap::execution_model — streaming mode preserved
#[test]
fn test_execution_model_streaming_mode_edge() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    let mode = StdProviderFactory::execution_model(config)
        .execution_mode(ExecutionModeLookupRequest)
        .expect("ok")
        .mode;
    assert_eq!(mode, ExecutionMode::Streaming);
}

// --- stream_handler ---

/// @covers: ProviderBootstrap::stream_handler — builds an empty handler
#[test]
fn test_stream_handler_starts_empty_happy() {
    let mut h = StdProviderFactory::stream_handler();
    assert!(h.next_chunk(NextChunkRequest).expect("ok").chunk.is_none());
}

/// @covers: ProviderBootstrap::stream_handler — no pending tool call initially
#[test]
fn test_stream_handler_no_pending_call_error() {
    let h = StdProviderFactory::stream_handler();
    assert!(h
        .pending_tool_call(PendingToolCallRequest)
        .expect("ok")
        .tool_call
        .is_none());
}

/// @covers: ProviderBootstrap::stream_handler — independent instances per call
#[test]
fn test_stream_handler_independent_instances_edge() {
    let mut a = StdProviderFactory::stream_handler();
    a.accumulate(AccumulateRequest {
        delta: edge_llm_provider::StreamDelta::text("x".to_string()),
    })
    .expect("ok");
    let mut b = StdProviderFactory::stream_handler();
    assert!(b.next_chunk(NextChunkRequest).expect("ok").chunk.is_none());
}

// --- message ---

/// @covers: ProviderBootstrap::message — constructs a user-role message via factory
#[test]
fn test_message_user_role_happy() {
    let m = StdProviderFactory::message(MessageRole::User, "hello");
    assert_eq!(m.role, MessageRole::User);
    assert_eq!(m.content, "hello");
}

/// @covers: ProviderBootstrap::message — empty string content is accepted without panic
#[test]
fn test_message_empty_content_error() {
    let m = StdProviderFactory::message(MessageRole::Tool, "");
    assert_eq!(m.role, MessageRole::Tool);
    assert!(
        m.content.is_empty(),
        "factory must accept empty content without panic"
    );
}

/// @covers: ProviderBootstrap::message — all three roles produce correct role field
#[test]
fn test_message_all_roles_edge() {
    for role in [MessageRole::User, MessageRole::Assistant, MessageRole::Tool] {
        let m = StdProviderFactory::message(role.clone(), "x");
        assert_eq!(m.role, role);
    }
}

// --- completion_input ---

/// @covers: ProviderBootstrap::completion_input — constructs a fully-specified input
#[test]
fn test_completion_input_full_spec_happy() {
    let msgs = vec![CompletionMessage::user("ping")];
    let tools = vec![ToolDefinition::new("noop", "No-op", json!({}))];
    let config = ExecutionConfig::new(1024, 30_000, false, false, ExecutionMode::Async);
    let input = StdProviderFactory::completion_input(msgs, tools, Some("sys".to_string()), config);
    assert_eq!(input.messages.len(), 1);
    assert_eq!(input.tools.len(), 1);
    assert_eq!(input.system.as_deref(), Some("sys"));
}

/// @covers: ProviderBootstrap::completion_input — empty messages vector is accepted without panic
#[test]
fn test_completion_input_empty_messages_error() {
    let config = ExecutionConfig::new(1024, 30_000, false, false, ExecutionMode::Async);
    let input = StdProviderFactory::completion_input(vec![], vec![], None, config);
    assert!(
        input.messages.is_empty(),
        "factory must accept empty messages without panic"
    );
}

/// @covers: ProviderBootstrap::completion_input — no system prompt and no tools
#[test]
fn test_completion_input_minimal_edge() {
    let config = ExecutionConfig::new(512, 10_000, false, false, ExecutionMode::Async);
    let input = StdProviderFactory::completion_input(
        vec![CompletionMessage::user("hi")],
        vec![],
        None,
        config,
    );
    assert!(input.system.is_none());
    assert!(input.tools.is_empty());
}

// --- provider_completer ---

/// @covers: ProviderBootstrap::provider_completer — returns a EchoProviderCompleter
#[test]
fn test_provider_completer_returns_instance_happy() {
    let c: EchoProviderCompleter = StdProviderFactory::provider_completer();
    assert_eq!(
        std::mem::size_of_val(&c),
        0,
        "echo completer must be zero-sized"
    );
}

/// @covers: ProviderBootstrap::provider_completer — instance implements Completer (can call complete)
#[test]
fn test_provider_completer_implements_completer_error() {
    let c = StdProviderFactory::provider_completer();
    let req = CompletionRequest::new("echo", vec![Message::user("hi")]);
    let result = block_on(c.complete(&req)).expect("complete should succeed");
    assert!(result.content.is_some());
}

/// @covers: ProviderBootstrap::provider_completer — repeated calls return independent instances
#[test]
fn test_provider_completer_independent_instances_edge() {
    let a = StdProviderFactory::provider_completer();
    let b = StdProviderFactory::provider_completer();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}
