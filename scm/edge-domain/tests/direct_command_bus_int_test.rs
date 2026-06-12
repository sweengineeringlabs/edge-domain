//! Coverage for api/command/direct_command_bus.rs and api/command/types/direct_command_bus.rs
use edge_domain::{CommandBus, DirectCommandBus, Domain};
use std::sync::Arc;

#[test]
fn test_direct_command_bus_type_satisfies_command_bus_trait() {
    let bus: Arc<dyn CommandBus> = Domain::direct_command_bus();
    drop(bus);
}

#[test]
fn test_direct_command_bus_marker_type_is_constructible() {
    let _marker = DirectCommandBus;
}

#[test]
fn test_direct_command_bus_factory_returns_arc_command_bus() {
    let _: Arc<dyn CommandBus> = Domain::direct_command_bus();
}
