//! SAF facade tests — `QueryBus` trait via `DirectQueryBus`.

use edge_domain_query::{DirectQueryBus, Query, QueryBus, QueryBusBootstrap, QueryError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Buses;
impl QueryBusBootstrap for Buses {}

struct Ok_(String);
impl Query for Ok_ {
    type Result = String;

    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

struct Err_;
impl Query for Err_ {
    type Result = String;

    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Err(QueryError::Internal("boom".into())) })
    }
}

/// @covers: QueryBus::dispatch — success
#[test]
fn test_dispatch_ok_query_returns_result_happy() {
    let bus: DirectQueryBus<String> = Buses::direct();
    let result = block_on(bus.dispatch(Box::new(Ok_("pong".into()))));
    assert_eq!(result.unwrap(), "pong");
}

/// @covers: QueryBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_query_returns_err_error() {
    let bus: DirectQueryBus<String> = Buses::direct();
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_err());
}

/// @covers: QueryBus::dispatch — multiple dispatches independent
#[test]
fn test_dispatch_multiple_sequential_queries_are_independent_edge() {
    let bus: DirectQueryBus<String> = Buses::direct();
    assert!(block_on(bus.dispatch(Box::new(Ok_("a".into())))).is_ok());
    assert!(block_on(bus.dispatch(Box::new(Err_))).is_err());
    assert!(block_on(bus.dispatch(Box::new(Ok_("b".into())))).is_ok());
}
