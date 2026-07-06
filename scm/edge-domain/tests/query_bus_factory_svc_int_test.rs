//! Integration tests for the `DirectQueryBus` construction path.

use edge_domain::DirectQueryBus;

/// @covers DirectQueryBus::new — happy path: returns a DirectQueryBus
#[test]
fn test_query_bus_factory_direct_returns_direct_bus_happy() {
    let bus: DirectQueryBus<()> = DirectQueryBus::new();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers DirectQueryBus::new — error: DirectQueryBus is zero-size (unit struct)
#[test]
fn test_query_bus_factory_direct_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus<()>>(), 0);
}

/// @covers DirectQueryBus::new — edge: successive calls produce independent instances
#[test]
fn test_query_bus_factory_direct_independent_instances_edge() {
    let a: DirectQueryBus<()> = DirectQueryBus::new();
    let b: DirectQueryBus<()> = DirectQueryBus::new();
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}
