//! SAF facade tests — `CommandBus` trait via `DirectCommandBus`.

use edge_domain_command::{Command, CommandBus, CommandBusBootstrap, CommandError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Buses;
impl CommandBusBootstrap for Buses {}

struct Ok_;
impl Command for Ok_ {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::Internal("boom".into())) })
    }
}

/// @covers: CommandBus::dispatch — success
#[test]
fn test_dispatch_ok_command_returns_ok_happy() {
    let bus = Buses::direct();
    assert!(block_on(bus.dispatch(Box::new(Ok_))).is_ok());
}

/// @covers: CommandBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_command_returns_err_error() {
    let bus = Buses::direct();
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_err());
}

/// @covers: CommandBus::dispatch — multiple dispatches independent
#[test]
fn test_dispatch_multiple_sequential_commands_are_independent_edge() {
    let bus = Buses::direct();
    assert!(block_on(bus.dispatch(Box::new(Ok_))).is_ok());
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_err());
    assert!(block_on(bus.dispatch(Box::new(Ok_))).is_ok());
}
