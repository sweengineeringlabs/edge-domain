//! Handler integration tests — `prompt_handler` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_observe::StdObserveFactory;
use edge_domain_security::SecurityContext;
use edge_llm_prompt::{StdPromptFactory, PromptMetadata, RenderContext, Variable, VariableType};
use futures::executor::block_on;

fn make_handler() -> impl Handler<Request = RenderContext, Response = String> {
    let var = Variable::new("name".to_string(), VariableType::String);
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vec![var],
    );
    StdPromptFactory::default_prompt_handler("Hello {{name}}".to_string(), metadata)
}

/// @covers: prompt_handler (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_renders_template_happy() {
    let h = make_handler();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let hctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(&h, context, hctx)).expect("handler ok");
    assert_eq!(out, "Hello Ada");
}

/// @covers: prompt_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = make_handler();
    assert_eq!(Handler::id(&h), "prompt.render");
}

/// @covers: prompt_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = make_handler();
    assert_eq!(Handler::pattern(&h), "prompt/render");
}

/// @covers: prompt_handler — a missing required variable surfaces an error through the pipeline
#[test]
fn test_handler_execute_missing_variable_errors_error() {
    let h = make_handler();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let hctx = HandlerContext::new(&security, &commands, observer.as_ref());
    assert!(block_on(Handler::execute(&h, RenderContext::new(), hctx)).is_err());
}

/// @covers: prompt_handler — handler constructed from factory works end-to-end
#[test]
fn test_factory_handler_renders_through_handler_happy() {
    let h = make_handler();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let hctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(&h, context, hctx)).expect("ok");
    assert_eq!(out, "Hello Ada");
}
