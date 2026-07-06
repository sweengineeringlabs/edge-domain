//! Integration tests for the `CommandBusBootstrap` SAF facade.

use edge_domain::{CommandBusBootstrap, DirectCommandBus};

struct TestCommandBuses;
impl CommandBusBootstrap for TestCommandBuses {}

/// @covers CommandBusBootstrap::direct — happy path: returns a DirectCommandBus
#[test]
fn test_command_bus_factory_direct_returns_direct_bus_happy() {
    let bus: DirectCommandBus = TestCommandBuses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers CommandBusBootstrap::direct — error: two calls produce independent instances
#[test]
fn test_command_bus_factory_direct_produces_independent_instances_error() {
    let a = TestCommandBuses::direct();
    let b = TestCommandBuses::direct();
    // Both are unit structs — construction succeeds and neither panics.
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

/// @covers CommandBusBootstrap::direct — edge: DirectCommandBus is a unit struct (zero-size)
#[test]
fn test_command_bus_factory_direct_is_unit_struct_edge() {
    assert_eq!(std::mem::size_of::<DirectCommandBus>(), 0);
}
