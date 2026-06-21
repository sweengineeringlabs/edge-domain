//! Integration tests — `HandlerContext` type.

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observe::{ObserveContext, StdObserveFactory};
use edge_domain_security::SecurityContext;

/// @covers: HandlerContext::new — constructs with unauthenticated security and direct bus
#[test]
fn test_handler_context_constructs_with_unauthenticated_security_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // security accessor is available
    let _ = ctx.security();
}

/// @covers: HandlerContext::commands — dispatch on direct bus succeeds
#[test]
fn test_handler_context_commands_field_is_accessible_error() {
    use edge_domain_command::NoopCommand;
    use futures::executor::block_on;

    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // dispatch through ctx.commands() must not panic — direct bus dispatches inline
    let result = block_on(ctx.commands().dispatch(Box::new(NoopCommand)));
    assert!(result.is_ok());
}

/// @covers: HandlerContext — Copy semantics allow multiple uses without move
#[test]
fn test_handler_context_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let ctx2 = ctx; // Copy — ctx is still usable
    let _ = (ctx.security(), ctx2.security());
}

// ── observer accessor ─────────────────────────────────────────────────────────

/// @covers: HandlerContext::observer — returns bound ObserveContext
#[test]
fn test_observer_returns_bound_observe_context_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    ctx.observer().tracer().start_span("h", "op").finish();
}

/// @covers: HandlerContext::observer — tracer usable across multiple spans
#[test]
fn test_observer_tracer_usable_after_construction_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    for _ in 0..3 {
        ctx.observer().tracer().start_span("h", "op").finish();
    }
}

/// @covers: HandlerContext::observer — empty span ids do not panic
#[test]
fn test_observer_empty_span_ids_no_panic_error() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    ctx.observer().tracer().start_span("", "").finish();
}

/// @covers: HandlerContext — Copy semantics preserved with observer field
#[test]
fn test_handler_context_with_observer_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let ctx2 = ctx; // Copy — ctx still usable
                    // Both copies can reach the observer
    let _: &dyn ObserveContext = ctx.observer();
    let _: &dyn ObserveContext = ctx2.observer();
}
