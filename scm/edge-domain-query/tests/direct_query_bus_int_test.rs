//! Integration tests for `DirectQueryBus` — the zero-size in-process query bus marker.

use edge_domain_query::{DirectQueryBus, Query, QueryBus, QueryError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_(String);
impl Query<String> for Ok_ {
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

struct Err_;
impl Query<String> for Err_ {
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("gone".into())) })
    }
}

/// @covers: DirectQueryBus — is a zero-sized type
#[test]
fn test_direct_query_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DirectQueryBus>(), 0);
}

/// @covers: DirectQueryBus — dispatches failing query
#[test]
fn test_direct_query_bus_dispatch_error_query_returns_err_error() {
    let result = block_on(DirectQueryBus.dispatch(Box::new(Err_)));
    assert!(result.is_err());
}

/// @covers: DirectQueryBus — usable as `&dyn QueryBus<R>`
#[test]
fn test_direct_query_bus_dyn_dispatch_returns_ok_edge() {
    let bus: &dyn QueryBus<String> = &DirectQueryBus;
    let result = block_on(bus.dispatch(Box::new(Ok_("hi".into()))));
    assert_eq!(result.unwrap(), "hi");
}
