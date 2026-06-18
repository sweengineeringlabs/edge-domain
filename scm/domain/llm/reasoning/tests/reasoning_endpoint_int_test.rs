//! Handler integration tests — `ReasoningEndpoint` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_reasoning::{ReasoningEndpoint, ReasoningFactory, ReasoningPattern, StdReasoningFactory};
use futures::executor::block_on;

fn endpoint() -> ReasoningEndpoint {
    StdReasoningFactory::endpoint(ReasoningPattern::ChainOfThought)
}

/// @covers: ReasoningEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_returns_complete_process_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    let out = block_on(Handler::execute(&ep, "solve x".to_string(), ctx)).expect("handler ok");
    assert!(out.is_complete);
    assert_eq!(out.step_count(), 3);
}

/// @covers: ReasoningEndpoint — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    assert_eq!(Handler::id(&endpoint()), "reasoning.reason");
}

/// @covers: ReasoningEndpoint — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    assert_eq!(Handler::pattern(&endpoint()), "reasoning/reason");
}

/// @covers: ReasoningEndpoint — blank problem surfaces an error through the pipeline
#[test]
fn test_handler_execute_blank_problem_errors_error() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&ep, "   ".to_string(), ctx)).is_err());
}
