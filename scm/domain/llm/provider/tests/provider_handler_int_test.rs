//! Handler integration tests — `provider_handler` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::DirectCommandBus;
use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext, IdRequest, PatternRequest};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_provider::{ExecutionConfig, ExecutionMode, StdProviderFactory};
use futures::executor::block_on;

fn make_config() -> ExecutionConfig {
    ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async)
}

/// @covers: provider_handler (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let h = StdProviderFactory::default_provider_handler(make_config());
    let security: SecurityContext = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &h,
        ExecutionRequest {
            req: "ship it".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("handler ok");
    assert!(out.reasoning.contains("ship it"));
}

/// @covers: provider_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = StdProviderFactory::default_provider_handler(make_config());
    assert_eq!(
        Handler::id(&h, IdRequest).expect("id ok").id,
        "provider.execute_step"
    );
}

/// @covers: provider_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = StdProviderFactory::default_provider_handler(make_config());
    assert_eq!(
        Handler::pattern(&h, PatternRequest)
            .expect("pattern ok")
            .pattern,
        "provider/execute_step"
    );
}
