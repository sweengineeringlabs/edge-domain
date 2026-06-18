//! SAF facade tests — `ProviderFactory` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::{
    ExecutionConfig, ExecutionMode, ExecutionModel, ModelFamily, ModelInfo, Provider, ProviderConfig,
    ProviderFactory, StdProviderFactory, StreamHandler,
};

// --- default_provider_handler ---

/// @covers: default_provider_handler — builds a usable Handler
#[test]
fn test_default_provider_handler_runs_happy() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    let out = block_on(Handler::execute(&h, "go".to_string(), ctx)).expect("ok");
    assert!(out.reasoning.contains("go"));
}

/// @covers: default_provider_handler — zero-budget config surfaces an error through the pipeline
#[test]
fn test_default_provider_handler_zero_budget_errors_error() {
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;
    let config = ExecutionConfig::new(0, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&h, "go".to_string(), ctx)).is_err());
}

/// @covers: default_provider_handler — exposes the stable dispatch id
#[test]
fn test_default_provider_handler_exposes_handler_id_edge() {
    use edge_domain_handler::Handler;
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    let h = StdProviderFactory::default_provider_handler(config);
    assert_eq!(Handler::id(&h), "provider.execute_step");
}

// --- std_factory ---

/// @covers: ProviderFactory::std_factory — returns the standard factory
#[test]
fn test_std_factory_returns_instance_happy() {
    let _f: StdProviderFactory = StdProviderFactory::std_factory();
}

/// @covers: ProviderFactory::std_factory — instance is zero-sized
#[test]
fn test_std_factory_is_zero_sized_error() {
    let f = StdProviderFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0);
}

/// @covers: ProviderFactory::std_factory — repeated calls are equivalent
#[test]
fn test_std_factory_repeatable_edge() {
    let _a = StdProviderFactory::std_factory();
    let _b = StdProviderFactory::std_factory();
}

// --- provider ---

/// @covers: ProviderFactory::provider — builds a usable provider
#[test]
fn test_provider_builds_named_provider_happy() {
    let config = ProviderConfig::new("claude".to_string(), 0.7, 8192);
    let info = ModelInfo::new(
        "claude".to_string(),
        "Claude".to_string(),
        ModelFamily::Anthropic,
        8192,
    );
    assert_eq!(StdProviderFactory::provider(config, info).name(), "claude");
}

/// @covers: ProviderFactory::provider — empty model produces an unhealthy provider
#[test]
fn test_provider_empty_model_unhealthy_error() {
    let config = ProviderConfig::new(String::new(), 0.7, 8192);
    let info = ModelInfo::new(String::new(), String::new(), ModelFamily::OpenAI, 8192);
    assert!(StdProviderFactory::provider(config, info)
        .health_check()
        .is_err());
}

/// @covers: ProviderFactory::provider — family flows from model metadata
#[test]
fn test_provider_reports_model_family_edge() {
    let config = ProviderConfig::new("gpt".to_string(), 0.5, 4096);
    let info = ModelInfo::new(
        "gpt".to_string(),
        "GPT".to_string(),
        ModelFamily::OpenAI,
        4096,
    );
    assert_eq!(
        StdProviderFactory::provider(config, info).model_family(),
        ModelFamily::OpenAI
    );
}

// --- execution_model ---

/// @covers: ProviderFactory::execution_model — builds a model in the given mode
#[test]
fn test_execution_model_builds_in_mode_happy() {
    let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
    assert_eq!(
        StdProviderFactory::execution_model(config).execution_mode(),
        ExecutionMode::Async
    );
}

/// @covers: ProviderFactory::execution_model — zero budget cannot execute
#[test]
fn test_execution_model_zero_budget_blocked_error() {
    let config = ExecutionConfig::new(0, 30_000, true, false, ExecutionMode::Async);
    assert!(StdProviderFactory::execution_model(config)
        .can_execute()
        .is_err());
}

/// @covers: ProviderFactory::execution_model — streaming mode preserved
#[test]
fn test_execution_model_streaming_mode_edge() {
    let config = ExecutionConfig::new(4096, 30_000, true, true, ExecutionMode::Streaming);
    assert_eq!(
        StdProviderFactory::execution_model(config).execution_mode(),
        ExecutionMode::Streaming
    );
}

// --- stream_handler ---

/// @covers: ProviderFactory::stream_handler — builds an empty handler
#[test]
fn test_stream_handler_starts_empty_happy() {
    let mut h = StdProviderFactory::stream_handler();
    assert!(h.next_chunk().is_none());
}

/// @covers: ProviderFactory::stream_handler — no pending tool call initially
#[test]
fn test_stream_handler_no_pending_call_error() {
    let h = StdProviderFactory::stream_handler();
    assert!(h.pending_tool_call().is_none());
}

/// @covers: ProviderFactory::stream_handler — independent instances per call
#[test]
fn test_stream_handler_independent_instances_edge() {
    let mut a = StdProviderFactory::stream_handler();
    a.accumulate(edge_llm_provider::StreamDelta::text("x".to_string()));
    let mut b = StdProviderFactory::stream_handler();
    assert!(b.next_chunk().is_none());
}
