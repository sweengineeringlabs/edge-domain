//! Integration tests — `HandlerContext` type.

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;

/// @covers: HandlerContext — constructs with unauthenticated security and direct bus
#[test]
fn test_handler_context_constructs_with_unauthenticated_security_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    // Verify the context holds a reference to the same security object
    assert!(std::ptr::eq(ctx.security, &security));
}

/// @covers: HandlerContext::commands — dispatch on direct bus succeeds
#[test]
fn test_handler_context_commands_field_is_accessible_error() {
    use edge_domain_command::NoopCommand;
    use futures::executor::block_on;

    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    let result = block_on(ctx.commands.dispatch(Box::new(NoopCommand)));
    assert!(result.is_ok());
    assert!(std::ptr::eq(ctx.security, &security));
}

/// @covers: HandlerContext — Copy semantics allow multiple uses without move
#[test]
fn test_handler_context_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    let ctx2 = ctx;
    assert!(std::ptr::eq(ctx.security, &security));
    assert!(std::ptr::eq(ctx2.security, &security));
}

/// @covers: HandlerContext::observer — returns bound ObserverContext
#[test]
fn test_observer_returns_bound_observe_context_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    ctx.observer.tracer().start_span("h", "op").finish();
    assert!(std::ptr::eq(ctx.security, &security));
}

/// @covers: HandlerContext::observer — tracer usable across multiple spans
#[test]
fn test_observer_tracer_usable_after_construction_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    for i in 0..3 {
        ctx.observer.tracer().start_span(&format!("span_{i}"), "op").finish();
    }
    assert!(std::ptr::eq(ctx.security, &security));
}

/// @covers: HandlerContext::observer — empty span ids do not panic
#[test]
fn test_observer_empty_span_ids_no_panic_error() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    ctx.observer.tracer().start_span("", "").finish();
    assert!(std::ptr::eq(ctx.security, &security));
}

/// @covers: HandlerContext — Copy semantics preserved with observer field
#[test]
fn test_handler_context_with_observer_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    let ctx2 = ctx;
    ctx.observer.tracer().start_span("ctx1", "op").finish();
    ctx2.observer.tracer().start_span("ctx2", "op").finish();
    assert!(std::ptr::eq(ctx.security, &security));
}
