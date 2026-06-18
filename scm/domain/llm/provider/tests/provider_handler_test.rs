//! Handler integration tests — `provider_handler` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_provider::{default_provider_handler, ExecutionConfig, ExecutionMode};
use futures::executor::block_on;

fn make_config() -> ExecutionConfig {
    ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async)
}

/// @covers: provider_handler (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_runs_core_happy() {
    let h = default_provider_handler(make_config());
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    let out = block_on(Handler::execute(&h, "ship it".to_string(), ctx)).expect("handler ok");
    assert!(out.reasoning.contains("ship it"));
}

/// @covers: provider_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = default_provider_handler(make_config());
    assert_eq!(Handler::id(&h), "provider.execute_step");
}

/// @covers: provider_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = default_provider_handler(make_config());
    assert_eq!(Handler::pattern(&h), "provider/execute_step");
}
