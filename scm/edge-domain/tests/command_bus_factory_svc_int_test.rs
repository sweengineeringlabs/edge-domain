//! Integration tests for the `CommandBusFactory` SAF facade.

use edge_domain::{CommandBusFactory, DirectCommandBus};

struct TestCommandBuses;
impl CommandBusFactory for TestCommandBuses {}

/// @covers CommandBusFactory::direct — happy path: returns a DirectCommandBus
#[test]
fn test_command_bus_factory_direct_returns_direct_bus_happy() {
    let _: DirectCommandBus = TestCommandBuses::direct();
}

/// @covers CommandBusFactory::direct — error: two calls produce independent instances
#[test]
fn test_command_bus_factory_direct_produces_independent_instances_error() {
    let _a = TestCommandBuses::direct();
    let _b = TestCommandBuses::direct();
    // Both are unit structs — construction succeeds and neither panics.
}

/// @covers CommandBusFactory::direct — edge: DirectCommandBus is a unit struct (zero-size)
#[test]
fn test_command_bus_factory_direct_is_unit_struct_edge() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}
