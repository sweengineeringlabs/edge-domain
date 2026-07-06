//! SAF service tests — `provider_handler` and `default_provider_handler` factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext, IdRequest};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_provider::{EchoExecutionModel, ExecutionConfig, ExecutionMode, StdProviderFactory};
use futures::executor::block_on;
use std::sync::Arc;

fn make_config(max_tokens: u32) -> ExecutionConfig {
    ExecutionConfig::new(max_tokens, 30_000, true, false, ExecutionMode::Async)
}

// --- provider_handler ---

/// @covers: StdProviderFactory::provider_handler — executes a step via the Handler face
#[test]
fn test_provider_handler_executes_step_happy() {
    let model = Arc::new(EchoExecutionModel::new(make_config(4096)));
    let h = StdProviderFactory::provider_handler(model);
    let security: SecurityContext = SecurityContext::unauthenticated();
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
            req: "hello".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert!(!out.reasoning.is_empty());
}

/// @covers: StdProviderFactory::provider_handler — zero token budget surfaces an error
#[test]
fn test_provider_handler_zero_budget_error() {
    let model = Arc::new(EchoExecutionModel::new(make_config(0)));
    let h = StdProviderFactory::provider_handler(model);
    let security: SecurityContext = SecurityContext::unauthenticated();
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
            req: "hello".to_string(),
            ctx: &ctx,
        },
    ))
    .is_err());
}

/// @covers: StdProviderFactory::provider_handler — exposes stable dispatch id
#[test]
fn test_provider_handler_stable_dispatch_id_edge() {
    let model = Arc::new(EchoExecutionModel::new(make_config(4096)));
    let h = StdProviderFactory::provider_handler(model);
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id must succeed").id,
        "provider.execute_step"
    );
}

// --- default_provider_handler ---

/// @covers: StdProviderFactory::default_provider_handler — runs via the echo model
#[test]
fn test_default_provider_handler_runs_happy() {
    let h = StdProviderFactory::default_provider_handler(make_config(4096));
    let security: SecurityContext = SecurityContext::unauthenticated();
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
            req: "prompt".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert!(!out.reasoning.is_empty());
}

/// @covers: StdProviderFactory::default_provider_handler — zero budget surfaces an error
#[test]
fn test_default_provider_handler_zero_budget_error() {
    let h = StdProviderFactory::default_provider_handler(make_config(0));
    let security: SecurityContext = SecurityContext::unauthenticated();
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
            req: "prompt".to_string(),
            ctx: &ctx,
        },
    ))
    .is_err());
}

/// @covers: StdProviderFactory::default_provider_handler — exposes stable dispatch id
#[test]
fn test_default_provider_handler_stable_id_edge() {
    let h = StdProviderFactory::default_provider_handler(make_config(4096));
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id must succeed").id,
        "provider.execute_step"
    );
}
