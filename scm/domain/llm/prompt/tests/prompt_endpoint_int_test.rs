//! Handler integration tests — `PromptEndpoint` as a dispatchable `Handler`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext};
use edge_domain_security::SecurityContext;
use edge_llm_prompt::{
    PromptEndpoint, PromptFactory, PromptMetadata, RenderContext, StdPromptFactory, Variable,
    VariableType,
};
use futures::executor::block_on;

fn endpoint() -> PromptEndpoint {
    let var = Variable::new("name".to_string(), VariableType::String);
    let metadata = PromptMetadata::new(
        "greet".to_string(),
        "Greeting".to_string(),
        "1".to_string(),
        vec![var],
    );
    StdPromptFactory::endpoint("Hello {{name}}".to_string(), metadata)
}

/// @covers: PromptEndpoint (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_renders_template_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let hctx = HandlerContext { security: &security, commands: &commands };
    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(&ep, context, hctx)).expect("handler ok");
    assert_eq!(out, "Hello Ada");
}

/// @covers: PromptEndpoint — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    assert_eq!(Handler::id(&endpoint()), "prompt.render");
}

/// @covers: PromptEndpoint — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    assert_eq!(Handler::pattern(&endpoint()), "prompt/render");
}

/// @covers: PromptEndpoint — a missing required variable surfaces an error through the pipeline
#[test]
fn test_handler_execute_missing_variable_errors_error() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let hctx = HandlerContext { security: &security, commands: &commands };
    assert!(block_on(Handler::execute(&ep, RenderContext::new(), hctx)).is_err());
}

/// @covers: PromptEndpoint — endpoint constructed from factory works end-to-end
#[test]
fn test_factory_endpoint_renders_through_handler_happy() {
    let ep = endpoint();
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let hctx = HandlerContext { security: &security, commands: &commands };
    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    let out = block_on(Handler::execute(&ep, context, hctx)).expect("ok");
    assert_eq!(out, "Hello Ada");
}
