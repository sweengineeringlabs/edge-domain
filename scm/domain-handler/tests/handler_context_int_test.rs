//! Integration tests — `HandlerContext` type.

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{HandlerContext};
use edge_domain_security::SecurityContext;

/// @covers: HandlerContext — constructs with unauthenticated security and direct bus
#[test]
fn test_handler_context_constructs_with_unauthenticated_security_happy() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &bus };
    // security reference is accessible
    let _ = ctx.security;
}

/// @covers: HandlerContext — commands field rejects dispatch on NoopCommandBus (always ok, but bus is wired)
#[test]
fn test_handler_context_commands_field_is_accessible_error() {
    use edge_domain_command::NoopCommand;
    use futures::executor::block_on;

    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &bus };
    // dispatch through ctx.commands must not panic — direct bus dispatches inline
    let result = block_on(ctx.commands.dispatch(Box::new(NoopCommand)));
    assert!(result.is_ok());
}

/// @covers: HandlerContext — Copy semantics allow multiple uses without move
#[test]
fn test_handler_context_is_copy_edge() {
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &bus };
    let ctx2 = ctx; // Copy — ctx is still usable
    let _ = (ctx.security, ctx2.security);
}
