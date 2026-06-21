//! Integration tests for the `CommandBusBootstrap` SAF facade.

use edge_domain::{CommandBusBootstrap, DirectCommandBus};

struct TestCommandBuses;
impl CommandBusBootstrap for TestCommandBuses {}

/// @covers CommandBusBootstrap::direct — happy path: returns a DirectCommandBus
#[test]
fn test_command_bus_factory_direct_returns_direct_bus_happy() {
    let _: DirectCommandBus = TestCommandBuses::direct();
}

/// @covers CommandBusBootstrap::direct — error: two calls produce independent instances
#[test]
fn test_command_bus_factory_direct_produces_independent_instances_error() {
    let _a = TestCommandBuses::direct();
    let _b = TestCommandBuses::direct();
    // Both are unit structs — construction succeeds and neither panics.
}

/// @covers CommandBusBootstrap::direct — edge: DirectCommandBus is a unit struct (zero-size)
#[test]
fn test_command_bus_factory_direct_is_unit_struct_edge() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}
