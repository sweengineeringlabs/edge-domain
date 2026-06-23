//! Integration tests for `LoggingQueryBus<R>` — delegates dispatch and logs outcome.

use std::sync::Arc;
use edge_domain_query::{
    DirectQueryBus, LoggingQueryBus, NoopQueryBus, Query, QueryBus, QueryError,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

fn noop_inner() -> Arc<dyn QueryBus<Result = String>> {
    Arc::new(NoopQueryBus::<String>::new())
}

fn direct_inner() -> Arc<dyn QueryBus<Result = String>> {
    Arc::new(DirectQueryBus::<String>::new())
}

struct OkQuery(String);
impl Query for OkQuery {
    type Result = String;
    fn name(&self) -> &str { "ok-query" }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

struct ErrQuery;
impl Query for ErrQuery {
    type Result = String;
    fn name(&self) -> &str { "err-query" }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("gone".into())) })
    }
}

/// @covers: LoggingQueryBus — constructed from noop inner via struct literal
#[test]
fn test_logging_query_bus_new_from_noop_inner_happy() {
    let bus = LoggingQueryBus::<String> { inner: noop_inner() };
    assert!(!Arc::ptr_eq(&bus.inner, &Arc::new(NoopQueryBus::<String>::new())));
}

/// @covers: LoggingQueryBus — dispatch ok query returns value
#[tokio::test]
async fn test_logging_query_bus_dispatch_ok_query_returns_value_happy() {
    let bus = LoggingQueryBus::<String> { inner: direct_inner() };
    let result = bus.dispatch(Box::new(OkQuery("pong".into()))).await;
    assert_eq!(result.unwrap(), "pong");
}

/// @covers: LoggingQueryBus — dispatch error query propagates err
#[tokio::test]
async fn test_logging_query_bus_dispatch_error_query_returns_err_error() {
    let bus = LoggingQueryBus::<String> { inner: noop_inner() };
    assert!(bus.dispatch(Box::new(ErrQuery)).await.is_err());
}

/// @covers: LoggingQueryBus — usable as `dyn QueryBus`
#[test]
fn test_logging_query_bus_dyn_dispatch_edge() {
    let bus: Arc<dyn QueryBus<Result = String>> =
        Arc::new(LoggingQueryBus::<String> { inner: direct_inner() });
    let result = block_on(bus.dispatch(Box::new(OkQuery("hi".into()))));
    assert_eq!(result.unwrap(), "hi");
}
