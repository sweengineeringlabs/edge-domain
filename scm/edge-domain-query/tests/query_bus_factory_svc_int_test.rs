//! SAF facade tests — `QueryBusFactory` constructors.

use edge_domain_query::{DirectQueryBus, QueryBusFactory};

struct Buses;
impl QueryBusFactory for Buses {}

/// @covers: QueryBusFactory::direct — returns a usable marker
#[test]
fn test_direct_returns_zero_sized_marker_happy() {
    let bus = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: QueryBusFactory::direct — zero-sized
#[test]
fn test_direct_type_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus>(), 0);
}

/// @covers: QueryBusFactory::direct — independent calls
#[test]
fn test_direct_independent_calls_return_same_type_edge() {
    let a = Buses::direct();
    let b = Buses::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}
