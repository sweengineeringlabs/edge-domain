#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Query trait is exported from the crate root.

use edge_domain::Query;
use edge_domain::QueryError;

struct Count(u32);
impl Query<u32> for Count {
    fn name(&self) -> &str {
        "count"
    }
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<u32, QueryError>> {
        let v = self.0;
        Box::pin(async move { Ok(v) })
    }
}

#[tokio::test]
async fn test_query_svc_facade_execute_returns_wrapped_value() {
    assert_eq!(Count(99).execute().await.unwrap(), 99);
}

#[tokio::test]
async fn test_query_svc_facade_name_returns_identifier() {
    assert_eq!(Count(0).name(), "count");
}
