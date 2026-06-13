//! Integration tests for `NoopQuery` — the no-op `Query` implementation.

use edge_domain_query::{NoopQuery, Query};
use futures::executor::block_on;

/// @covers: NoopQuery::execute — always returns Ok(())
#[test]
fn test_execute_noop_query_returns_ok_happy() {
    let result = block_on(NoopQuery.execute());
    assert!(result.is_ok());
}

/// @covers: NoopQuery::name — returns default "query"
#[test]
fn test_name_noop_query_returns_default_name_error() {
    assert_eq!(NoopQuery.name(), "query");
}

/// @covers: NoopQuery::execute — repeated calls are independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    assert!(block_on(NoopQuery.execute()).is_ok());
    assert!(block_on(NoopQuery.execute()).is_ok());
}
