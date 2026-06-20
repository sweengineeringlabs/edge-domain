//! Integration tests for `DirectCommandBus` — the zero-size in-process command bus marker.

use edge_domain_command::{Command, CommandBus, CommandBusBootstrap, CommandError, DirectCommandBus,
    StdCommandBusFactory};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

/// @covers: DirectCommandBus — is a zero-sized type
#[test]
fn test_direct_command_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}

/// @covers: DirectCommandBus — dispatches failing command
#[test]
fn test_direct_command_bus_dispatch_error_command_returns_err_error() {
    let bus = StdCommandBusFactory::direct();
    let result = block_on(bus.dispatch(Box::new(Err_)));
    assert!(result.is_err());
}

/// @covers: DirectCommandBus — usable as a `&dyn CommandBus`
#[test]
fn test_direct_command_bus_dyn_dispatch_returns_ok_edge() {
    let bus = StdCommandBusFactory::direct();
    let bus_ref: &dyn CommandBus = &bus;
    let result = block_on(bus_ref.dispatch(Box::new(Ok_)));
    assert!(result.is_ok());
}
