//! Integration tests for `StdQueryBusFactory`.

use edge_domain_query::{DirectQueryBus, QueryBusFactory, StdQueryBusFactory};

/// @covers: StdQueryBusFactory::direct — returns zero-sized DirectQueryBus
#[test]
fn test_direct_returns_zero_sized_direct_query_bus_happy() {
    let bus: DirectQueryBus<String> = StdQueryBusFactory::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: StdQueryBusFactory::direct — different result type is also zero-sized
#[test]
fn test_direct_u32_result_type_is_zero_sized_error() {
    let bus: DirectQueryBus<u32> = StdQueryBusFactory::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: StdQueryBusFactory::direct — independent calls produce same-sized type
#[test]
fn test_direct_independent_calls_produce_same_sized_type_edge() {
    let a: DirectQueryBus<String> = StdQueryBusFactory::direct();
    let b: DirectQueryBus<String> = StdQueryBusFactory::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}
