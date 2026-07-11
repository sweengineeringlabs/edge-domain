//! Coverage for api/command/direct_command_bus.rs and api/command/types/direct_command_bus.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_domain::DirectCommandBusRequest;
use edge_domain::DomainRuntime;
use edge_domain::{CommandBus, Domain};
use edge_domain_command::DirectCommandBus;
use std::sync::Arc;

#[test]
fn test_direct_command_bus_type_satisfies_command_bus_trait() {
    let bus: Arc<dyn CommandBus> = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    drop(bus);
}

#[test]
fn test_direct_command_bus_marker_type_is_constructible() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}

#[test]
fn test_direct_command_bus_factory_returns_arc_command_bus() {
    let _: Arc<dyn CommandBus> = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
}
