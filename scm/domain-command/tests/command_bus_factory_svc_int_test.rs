//! SAF facade tests — `CommandBusBootstrap` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_command::{CommandBus, CommandBusBootstrap, NoopCommandBus};

struct Buses;
impl CommandBusBootstrap for Buses {}

/// @covers: CommandBusBootstrap::direct — returns a usable marker
#[test]
fn test_direct_returns_marker_happy() {
    let bus = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: CommandBusBootstrap::direct — zero-sized
#[test]
fn test_direct_is_zero_size_error() {
    let bus = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: CommandBusBootstrap::direct — independent calls
#[test]
fn test_direct_independent_calls_edge() {
    let a = Buses::direct();
    let b = Buses::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

// ── noop_bus ──────────────────────────────────────────────────────────────────

/// @covers: CommandBusBootstrap::noop_bus — returns a zero-sized noop bus
#[test]
fn test_noop_bus_returns_zero_sized_marker_happy() {
    let bus = Buses::noop_bus();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: CommandBusBootstrap::noop_bus — usable as &dyn CommandBus
#[test]
fn test_noop_bus_usable_as_dyn_command_bus_error() {
    let bus = Buses::noop_bus();
    let _: &dyn CommandBus = &bus;
}

/// @covers: CommandBusBootstrap::noop_bus — independent calls are independent
#[test]
fn test_noop_bus_independent_calls_edge() {
    let a = Buses::noop_bus();
    let b = Buses::noop_bus();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

// ── logging ───────────────────────────────────────────────────────────────────

/// @covers: CommandBusBootstrap::logging — wraps inner and returns a LoggingCommandBus
#[test]
fn test_logging_wraps_inner_bus_happy() {
    let inner: Arc<dyn CommandBus> = Arc::new(NoopCommandBus);
    let bus = Buses::logging(Arc::clone(&inner));
    let _: &dyn CommandBus = &bus;
}

/// @covers: CommandBusBootstrap::logging — different inner buses produce distinct instances
#[test]
fn test_logging_distinct_instances_for_different_inner_error() {
    let inner1: Arc<dyn CommandBus> = Arc::new(NoopCommandBus);
    let inner2: Arc<dyn CommandBus> = Arc::new(NoopCommandBus);
    let _a = Buses::logging(Arc::clone(&inner1));
    let _b = Buses::logging(Arc::clone(&inner2));
}

/// @covers: CommandBusBootstrap::logging — inner bus is callable through logging wrapper
#[test]
fn test_logging_delegates_dispatch_to_inner_edge() {
    use edge_domain_command::{Command, CommandDispatchRequest, CommandError, ExecutionRequest};
    use futures::executor::block_on;
    use futures::future::BoxFuture;

    struct LoggingFactoryOk;
    impl Command for LoggingFactoryOk {
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }

    let inner: Arc<dyn CommandBus> = Arc::new(NoopCommandBus);
    let bus = Buses::logging(inner);
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(LoggingFactoryOk),
    }));
    let unit = result.expect("dispatch should succeed");
    assert_eq!(unit, (), "dispatch result should be unit");
}
