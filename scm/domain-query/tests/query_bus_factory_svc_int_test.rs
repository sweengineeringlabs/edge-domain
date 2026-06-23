//! SAF facade tests — `QueryBusBootstrap` constructors.

use edge_domain_query::{DirectQueryBus, NoopQuery, Query, QueryBusBootstrap, StdQueryBusFactory};
use futures::executor::block_on;

struct Buses;
impl QueryBusBootstrap for Buses {}

/// @covers: QueryBusBootstrap::direct — returns a zero-sized marker
#[test]
fn test_direct_returns_zero_sized_marker_happy() {
    let bus: DirectQueryBus<String> = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: QueryBusBootstrap::direct — zero-sized (PhantomData<fn() -> R> is zero-sized)
#[test]
fn test_direct_type_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<DirectQueryBus<String>>(), 0);
}

/// @covers: QueryBusBootstrap::direct — independent calls return same-sized type
#[test]
fn test_direct_independent_calls_return_same_type_edge() {
    let a: DirectQueryBus<String> = Buses::direct();
    let b: DirectQueryBus<String> = Buses::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

/// @covers: QueryBusBootstrap::std — returns StdQueryBusFactory
#[test]
fn test_std_returns_std_query_bus_factory_happy() {
    let factory = Buses::std();
    assert_eq!(std::mem::size_of_val(&factory), 0);
}

/// @covers: QueryBusBootstrap::std — StdQueryBusFactory is zero-sized
#[test]
fn test_std_factory_type_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<StdQueryBusFactory>(), 0);
}

/// @covers: QueryBusBootstrap::std — independent calls return same-sized type
#[test]
fn test_std_independent_calls_return_same_sized_type_edge() {
    let a = Buses::std();
    let b = Buses::std();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

/// @covers: QueryBusBootstrap::noop_query — returns a NoopQuery that succeeds
#[test]
fn test_noop_query_returns_ok_on_execute_happy() {
    let q = Buses::noop_query();
    let result = block_on(q.execute());
    assert_eq!(result, Ok(()), "noop query should succeed with unit result");
}

/// @covers: QueryBusBootstrap::noop_query — NoopQuery uses the default name
#[test]
fn test_noop_query_default_name_is_query_error() {
    let q: NoopQuery = Buses::noop_query();
    assert_eq!(q.name(), "query");
}

/// @covers: QueryBusBootstrap::noop_query — repeated noop_query calls are independent
#[test]
fn test_noop_query_repeated_calls_are_independent_edge() {
    let q1 = Buses::noop_query();
    let q2 = Buses::noop_query();
    assert_eq!(block_on(q1.execute()), Ok(()), "first noop query should succeed");
    assert_eq!(block_on(q2.execute()), Ok(()), "second noop query should succeed");
}

/// @covers: QueryBusBootstrap::noop_query_bus — returns a NoopQueryBus that returns NotFound
#[test]
fn test_noop_query_bus_returns_not_found_on_dispatch_happy() {
    use futures::future::BoxFuture;
    use edge_domain_query::{NoopQueryBus, Query, QueryBus, QueryError};
    struct StrQuery;
    impl Query for StrQuery {
        type Result = String;
        fn name(&self) -> &str { "str" }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            Box::pin(async { Ok("x".into()) })
        }
    }
    let bus: NoopQueryBus<String> = Buses::noop_query_bus();
    let result = block_on(bus.dispatch(Box::new(StrQuery)));
    assert!(result.is_err());
}

/// @covers: QueryBusBootstrap::noop_query_bus — usable as dyn QueryBus
#[test]
fn test_noop_query_bus_usable_as_dyn_query_bus_error() {
    use std::sync::Arc;
    use edge_domain_query::{NoopQueryBus, QueryBus};
    let bus: Arc<dyn QueryBus<Result = String>> = Arc::new(Buses::noop_query_bus::<String>());
    assert_eq!(std::mem::size_of::<NoopQueryBus<String>>(), 0);
    drop(bus);
}

/// @covers: QueryBusBootstrap::noop_query_bus — independent calls return same-sized type
#[test]
fn test_noop_query_bus_independent_calls_edge() {
    use edge_domain_query::NoopQueryBus;
    let a: NoopQueryBus<String> = Buses::noop_query_bus();
    let b: NoopQueryBus<u32> = Buses::noop_query_bus();
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

/// @covers: QueryBusBootstrap::logging_query — wraps inner bus and delegates dispatch
#[test]
fn test_logging_query_wraps_inner_bus_happy() {
    use std::sync::Arc;
    use futures::future::BoxFuture;
    use edge_domain_query::{LoggingQueryBus, Query, QueryBus, QueryError};

    struct OkQuery;
    impl Query for OkQuery {
        type Result = String;
        fn name(&self) -> &str { "ok" }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            Box::pin(async { Ok("hello".into()) })
        }
    }

    let inner: Arc<dyn QueryBus<Result = String>> = Arc::new(DirectQueryBus::<String>::new());
    let bus: LoggingQueryBus<String> = Buses::logging_query(inner);
    let result = block_on(bus.dispatch(Box::new(OkQuery)));
    assert_eq!(result.unwrap(), "hello");
}

/// @covers: QueryBusBootstrap::logging_query — distinct instances for different inner types
#[test]
fn test_logging_query_distinct_instances_for_different_inner_error() {
    use std::sync::Arc;
    use edge_domain_query::{LoggingQueryBus, NoopQueryBus, QueryBus};

    let inner_str: Arc<dyn QueryBus<Result = String>> = Arc::new(NoopQueryBus::new());
    let inner_u32: Arc<dyn QueryBus<Result = u32>> = Arc::new(NoopQueryBus::new());
    let _bus_str: LoggingQueryBus<String> = Buses::logging_query(inner_str);
    let _bus_u32: LoggingQueryBus<u32> = Buses::logging_query(inner_u32);
}

/// @covers: QueryBusBootstrap::logging_query — delegates dispatch to inner on err path
#[test]
fn test_logging_query_delegates_dispatch_to_inner_edge() {
    use std::sync::Arc;
    use futures::future::BoxFuture;
    use edge_domain_query::{LoggingQueryBus, Query, QueryBus, QueryError};

    struct ErrQuery;
    impl Query for ErrQuery {
        type Result = String;
        fn name(&self) -> &str { "err" }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            Box::pin(async { Err(QueryError::NotFound("gone".into())) })
        }
    }

    let inner: Arc<dyn QueryBus<Result = String>> = Arc::new(DirectQueryBus::<String>::new());
    let bus: LoggingQueryBus<String> = Buses::logging_query(inner);
    let result = block_on(bus.dispatch(Box::new(ErrQuery)));
    assert!(matches!(result, Err(QueryError::NotFound(_))), "should delegate to inner and return NotFound");
}
}
