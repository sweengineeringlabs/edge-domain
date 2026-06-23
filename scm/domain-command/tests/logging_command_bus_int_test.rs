//! Integration tests for `LoggingCommandBus` — wraps inner bus and logs via tracing.

use std::sync::Arc;

use edge_domain_command::{
    Command, CommandBus, CommandBusBootstrap, CommandError, LoggingCommandBus, NoopCommandBus,
    StdCommandBusFactory,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn name(&self) -> &str {
        "ok"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct Err_;
impl Command for Err_ {
    fn name(&self) -> &str {
        "err"
    }
    fn execute(&self) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("denied".into())) })
    }
}

fn noop_inner() -> Arc<dyn CommandBus> {
    Arc::new(NoopCommandBus)
}

fn direct_inner() -> Arc<dyn CommandBus> {
    Arc::new(StdCommandBusFactory::direct())
}

/// @covers: LoggingCommandBus — constructs directly from inner field
#[test]
fn test_logging_command_bus_new_from_noop_inner_happy() {
    let bus = LoggingCommandBus { inner: noop_inner() };
    let _: &dyn CommandBus = &bus;
}

/// @covers: LoggingCommandBus::dispatch — propagates Ok from inner bus
#[test]
fn test_logging_command_bus_dispatch_ok_command_returns_ok_happy() {
    let bus = StdCommandBusFactory::logging(noop_inner());
    assert_eq!(block_on(bus.dispatch(Box::new(Ok_))), Ok(()));
}

/// @covers: LoggingCommandBus::dispatch — propagates Err from inner bus
#[test]
fn test_logging_command_bus_dispatch_error_command_returns_err_error() {
    let bus = StdCommandBusFactory::logging(direct_inner());
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_err());
}

/// @covers: LoggingCommandBus — usable via dyn CommandBus reference
#[test]
fn test_logging_command_bus_dyn_dispatch_edge() {
    let bus = StdCommandBusFactory::logging(noop_inner());
    let bus_ref: &dyn CommandBus = &bus;
    assert_eq!(block_on(bus_ref.dispatch(Box::new(Ok_))), Ok(()));
}
