//! Integration tests for `NoopQueryBus<R>` — always returns `QueryError::NotFound`.

use edge_domain_query::{NoopQueryBus, Query, QueryBus, QueryError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct StrQuery;
impl Query for StrQuery {
    type Result = String;
    fn name(&self) -> &str { "str-query" }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Ok("x".into()) })
    }
}

/// @covers: NoopQueryBus — is a zero-sized type
#[test]
fn test_noop_query_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NoopQueryBus<String>>(), 0);
}

/// @covers: NoopQueryBus — dispatch returns QueryError::NotFound
#[test]
fn test_noop_query_bus_dispatch_returns_not_found_error() {
    let bus = NoopQueryBus::<String>::new();
    let result = block_on(bus.dispatch(Box::new(StrQuery)));
    match result {
        Err(QueryError::NotFound(_)) => {}
        other => panic!("expected NotFound, got {other:?}"),
    }
}

/// @covers: NoopQueryBus — usable as `dyn QueryBus`
#[test]
fn test_noop_query_bus_dyn_dispatch_returns_err_edge() {
    use std::sync::Arc;
    let bus: Arc<dyn QueryBus<Result = String>> = Arc::new(NoopQueryBus::new());
    assert!(block_on(bus.dispatch(Box::new(StrQuery))).is_err());
}
