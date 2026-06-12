//! SAF facade tests — `Query` trait.

use edge_domain_query::{Query, QueryError};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct GetStr(String);
impl Query<String> for GetStr {
    fn name(&self) -> &str {
        &self.0
    }
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

struct Fail;
impl Query<String> for Fail {
    fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("x".into())) })
    }
}

/// @covers: Query::name — configured name returned
#[test]
fn test_name_configured_value_returned_happy() {
    assert_eq!(GetStr("get-order".into()).name(), "get-order");
}

/// @covers: Query::name — default impl returns "query"
#[test]
fn test_name_default_impl_returns_query_error() {
    assert_eq!(Fail.name(), "query");
}

/// @covers: Query::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let q: &dyn Query<String> = &GetStr("x".into());
    assert_eq!(q.name(), "x");
}

/// @covers: Query::execute — success
#[test]
fn test_execute_ok_query_returns_ok_happy() {
    assert_eq!(block_on(GetStr("hi".into()).execute()).unwrap(), "hi");
}

/// @covers: Query::execute — failure propagates
#[test]
fn test_execute_failing_query_returns_err_error() {
    assert!(block_on(Fail.execute()).is_err());
}

/// @covers: Query::execute — repeated calls independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let q = GetStr("z".into());
    assert!(block_on(q.execute()).is_ok());
    assert!(block_on(q.execute()).is_ok());
}
