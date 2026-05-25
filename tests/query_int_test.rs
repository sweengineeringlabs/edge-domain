//! Tests for Query trait
use edge_domain::Query;
use futures::future::BoxFuture;

struct TestQuery;
impl Query for TestQuery {
    type Result = String;
    fn execute(&self) -> BoxFuture<'_, Result<Self::Result, edge_domain::QueryError>> {
        Box::pin(async { Ok("result".into()) })
    }
}

#[tokio::test]
async fn test_query_execute() {
    let q = TestQuery;
    let result = q.execute().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "result");
}
