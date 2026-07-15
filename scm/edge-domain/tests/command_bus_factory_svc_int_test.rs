//! SAF facade smoke test — `CommandBus` factory (`Domain.direct_command_bus`) is exported
//! from the crate root.
#![cfg(feature = "command")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application::DirectCommandBusRequest;
use edge_application::Domain;
use edge_application::DomainRuntime;
use edge_application_command::{Command, CommandDispatchRequest, CommandError, ExecutionRequest};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_;
impl Command for Ok_ {
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

/// @covers: Domain.direct_command_bus — happy path: dispatches a successful command
#[test]
fn test_direct_command_bus_factory_dispatches_ok_command_happy() {
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let result = block_on(bus.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}

/// @covers: Domain.direct_command_bus — error: each call returns an independent, usable bus
#[test]
fn test_direct_command_bus_factory_independent_calls_error() {
    let a = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let b = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(!Arc::ptr_eq(&a, &b));
}

/// @covers: Domain.direct_command_bus — edge: usable through an explicit `&dyn CommandBus` reference
#[test]
fn test_direct_command_bus_factory_returns_dyn_command_bus_edge() {
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let bus_ref: &dyn edge_application_command::CommandBus = bus.as_ref();
    let result = block_on(bus_ref.dispatch(CommandDispatchRequest {
        command: Box::new(Ok_),
    }));
    assert_eq!(result, Ok(()));
}
