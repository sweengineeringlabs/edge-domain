//! Integration tests for `NoopCommandBus` — discards every command silently.

use edge_domain_command::{
    Command, CommandBus, CommandBusBootstrap, CommandError, NoopCommandBus, StdCommandBusFactory,
};
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

/// @covers: NoopCommandBus — is a zero-sized type
#[test]
fn test_noop_command_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NoopCommandBus>(), 0);
}

/// @covers: NoopCommandBus::dispatch — returns Ok for a successful command
#[test]
fn test_noop_command_bus_dispatch_ok_command_returns_ok_happy() {
    let bus = StdCommandBusFactory::noop_bus();
    assert!(block_on(bus.dispatch(Box::new(Ok_))).is_ok());
}

/// @covers: NoopCommandBus::dispatch — returns Ok even for a failing command
#[test]
fn test_noop_command_bus_dispatch_error_command_still_returns_ok_error() {
    let bus = StdCommandBusFactory::noop_bus();
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_ok());
}

/// @covers: NoopCommandBus — usable via dyn CommandBus reference
#[test]
fn test_noop_command_bus_dyn_dispatch_returns_ok_edge() {
    let bus = NoopCommandBus;
    let bus_ref: &dyn CommandBus = &bus;
    assert!(block_on(bus_ref.dispatch(Box::new(Ok_))).is_ok());
}
