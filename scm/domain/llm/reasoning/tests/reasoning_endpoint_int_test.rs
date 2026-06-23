//! Handler integration tests — `reasoning_handler` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;
use edge_llm_reasoning::{StdReasoningFactory, ReasoningPattern};
use futures::executor::block_on;

/// @covers: reasoning_handler — runs core under a request context
#[test]
fn test_handler_execute_returns_complete_process_happy() {
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&h, "solve x".to_string(), ctx)).expect("handler ok");
    assert!(out.is_complete);
    assert_eq!(out.step_count(), 3);
}

/// @covers: reasoning_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    assert_eq!(Handler::id(&h), "reasoning.reason");
}

/// @covers: reasoning_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    assert_eq!(Handler::pattern(&h), "reasoning/reason");
}

/// @covers: reasoning_handler — blank problem surfaces an error through the pipeline
#[test]
fn test_handler_execute_blank_problem_errors_error() {
    let h = StdReasoningFactory::default_reasoning_handler(ReasoningPattern::ChainOfThought);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    assert!(block_on(Handler::execute(&h, "   ".to_string(), ctx)).is_err());
}
