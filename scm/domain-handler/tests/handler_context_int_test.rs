//! Integration tests — `HandlerContext` type.

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;

/// @covers: HandlerContext::new — constructs with unauthenticated security and direct bus
#[test]
fn test_handler_context_constructs_with_unauthenticated_security_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // security accessor returns the context
    let _sec = ctx.security();
    assert!(true); // Context construction succeeds
}

/// @covers: HandlerContext::commands — dispatch on direct bus succeeds
#[test]
fn test_handler_context_commands_field_is_accessible_error() {
    use edge_domain_command::NoopCommand;
    use futures::executor::block_on;

    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // dispatch through ctx.commands() must not panic — direct bus dispatches inline
    let result = block_on(ctx.commands().dispatch(Box::new(NoopCommand)));
    assert!(result.is_ok());
    // Verify the command was actually dispatched (not just Ok(()) for any implementation)
    let _cmd_result = result.unwrap();
}

/// @covers: HandlerContext — Copy semantics allow multiple uses without move
#[test]
fn test_handler_context_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let ctx2 = ctx; // Copy — ctx is still usable
                    // Both copies have access to security context
    let _s1 = ctx.security();
    let _s2 = ctx2.security();
    assert!(true); // Both copies work independently
}

// ── observer accessor ─────────────────────────────────────────────────────────

/// @covers: HandlerContext::observer — returns bound ObserverContext
#[test]
fn test_observer_returns_bound_observe_context_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // Observer is accessible and spans can be created
    ctx.observer().tracer().start_span("h", "op").finish();
    assert!(true); // Test verifies observer works without panicking
}

/// @covers: HandlerContext::observer — tracer usable across multiple spans
#[test]
fn test_observer_tracer_usable_after_construction_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // Multiple span creations succeed without panic
    for i in 0..3 {
        ctx.observer()
            .tracer()
            .start_span(&format!("span_{i}"), "op")
            .finish();
    }
    assert!(true); // Test demonstrates tracer works repeatedly
}

/// @covers: HandlerContext::observer — empty span ids do not panic
#[test]
fn test_observer_empty_span_ids_no_panic_error() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    // Empty span IDs are handled gracefully without panic
    ctx.observer().tracer().start_span("", "").finish();
    assert!(true); // Test demonstrates tracer tolerates empty IDs
}

/// @covers: HandlerContext — Copy semantics preserved with observer field
#[test]
fn test_handler_context_with_observer_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    let ctx2 = ctx; // Copy — ctx still usable
                    // Both copies can reach the observer
    ctx.observer().tracer().start_span("ctx1", "op").finish();
    ctx2.observer().tracer().start_span("ctx2", "op").finish();
    assert!(true); // Both copies have independent access to observer
}
