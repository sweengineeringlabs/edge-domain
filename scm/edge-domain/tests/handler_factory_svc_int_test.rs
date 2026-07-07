//! SAF facade smoke test — `Handler` factory (`Domain::echo_handler`) is exported
//! from the crate root.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{Domain, Handler, HandlerContext};
use edge_domain_handler::{ExecutionRequest, IdRequest, PatternRequest};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

/// @covers: Domain::echo_handler — happy path: id and pattern are set from the given strings
#[test]
fn test_echo_handler_factory_sets_id_and_pattern_happy() {
    let h = Domain::echo_handler::<String>("greeter", "/greet");
    assert_eq!(h.id(IdRequest).unwrap().id, "greeter");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/greet");
}

/// @covers: Domain::echo_handler — error: empty id/pattern strings are still accepted, not rejected
#[test]
fn test_echo_handler_factory_accepts_empty_strings_error() {
    let h = Domain::echo_handler::<String>("", "");
    assert_eq!(h.id(IdRequest).unwrap().id, "");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "");
}

/// @covers: Domain::echo_handler — edge: the constructed handler actually echoes its input
#[test]
fn test_echo_handler_factory_built_handler_echoes_request_edge() {
    let h = Domain::echo_handler::<String>("echo", "/echo");
    let security = SecurityContext::unauthenticated();
    let bus = Domain::direct_command_bus();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };
    let result = block_on(h.execute(ExecutionRequest {
        req: "hello".to_string(),
        ctx: &ctx,
    }))
    .unwrap();
    assert_eq!(result, "hello");
}
