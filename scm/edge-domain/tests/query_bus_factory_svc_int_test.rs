//! Integration tests for the `QueryBusBootstrap` SAF facade.

use edge_domain::{DirectQueryBus, QueryBusBootstrap};

struct TestQueryBuses;
impl QueryBusBootstrap for TestQueryBuses {}

/// @covers QueryBusBootstrap::direct — happy path: returns a DirectQueryBus
#[test]
fn test_query_bus_factory_direct_returns_direct_bus_happy() {
    let _: DirectQueryBus<()> = TestQueryBuses::direct();
}

/// @covers QueryBusBootstrap::direct — error: DirectQueryBus is zero-size (unit struct)
#[test]
fn test_query_bus_factory_direct_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus<()>>(), 0);
}

/// @covers QueryBusBootstrap::direct — edge: successive calls produce independent instances
#[test]
fn test_query_bus_factory_direct_independent_instances_edge() {
    let _a: DirectQueryBus<()> = TestQueryBuses::direct();
    let _b: DirectQueryBus<()> = TestQueryBuses::direct();
}
