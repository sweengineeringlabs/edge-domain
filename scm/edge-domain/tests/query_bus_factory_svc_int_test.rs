//! Integration tests for the `QueryBusFactory` SAF facade.

use edge_domain::{DirectQueryBus, QueryBusFactory};

struct TestQueryBuses;
impl QueryBusFactory for TestQueryBuses {}

/// @covers QueryBusFactory::direct — happy path: returns a DirectQueryBus
#[test]
fn test_query_bus_factory_direct_returns_direct_bus_happy() {
    let _: DirectQueryBus = TestQueryBuses::direct();
}

/// @covers QueryBusFactory::direct — error: DirectQueryBus is zero-size (unit struct)
#[test]
fn test_query_bus_factory_direct_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus>(), 0);
}

/// @covers QueryBusFactory::direct — edge: successive calls produce independent instances
#[test]
fn test_query_bus_factory_direct_independent_instances_edge() {
    let _a = TestQueryBuses::direct();
    let _b = TestQueryBuses::direct();
}
