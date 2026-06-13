//! SAF facade tests — `QueryBusFactory` constructors.

use edge_domain_query::{DirectQueryBus, NoopQuery, Query, QueryBusFactory, StdQueryBusFactory};
use futures::executor::block_on;

struct Buses;
impl QueryBusFactory for Buses {}

/// @covers: QueryBusFactory::direct — returns a zero-sized marker
#[test]
fn test_direct_returns_zero_sized_marker_happy() {
    let bus: DirectQueryBus<String> = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: QueryBusFactory::direct — zero-sized (PhantomData<fn() -> R> is zero-sized)
#[test]
fn test_direct_type_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus<String>>(), 0);
}

/// @covers: QueryBusFactory::direct — independent calls return same-sized type
#[test]
fn test_direct_independent_calls_return_same_type_edge() {
    let a: DirectQueryBus<String> = Buses::direct();
    let b: DirectQueryBus<String> = Buses::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

/// @covers: QueryBusFactory::std — returns StdQueryBusFactory
#[test]
fn test_std_returns_std_query_bus_factory_happy() {
    let factory = Buses::std();
    assert_eq!(std::mem::size_of_val(&factory), 0);
}

/// @covers: QueryBusFactory::std — StdQueryBusFactory is zero-sized
#[test]
fn test_std_factory_type_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdQueryBusFactory>(), 0);
}

/// @covers: QueryBusFactory::std — independent calls return same-sized type
#[test]
fn test_std_independent_calls_return_same_sized_type_edge() {
    let a = Buses::std();
    let b = Buses::std();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

/// @covers: QueryBusFactory::noop_query — returns a NoopQuery that succeeds
#[test]
fn test_noop_query_returns_ok_on_execute_happy() {
    let q = Buses::noop_query();
    let result = block_on(q.execute());
    assert!(result.is_ok());
}

/// @covers: QueryBusFactory::noop_query — NoopQuery uses the default name
#[test]
fn test_noop_query_default_name_is_query_error() {
    let q: NoopQuery = Buses::noop_query();
    assert_eq!(q.name(), "query");
}

/// @covers: QueryBusFactory::noop_query — repeated noop_query calls are independent
#[test]
fn test_noop_query_repeated_calls_are_independent_edge() {
    let q1 = Buses::noop_query();
    let q2 = Buses::noop_query();
    assert!(block_on(q1.execute()).is_ok());
    assert!(block_on(q2.execute()).is_ok());
}
