#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Query trait is exported from the crate root.
#![cfg(feature = "query")]

use edge_application::Query;
use edge_application::QueryError;
use edge_application::QueryExecuteRequest;
use edge_application::QueryNameRequest;
use edge_application::QueryResultResponse;

struct Count(u32);
impl Query for Count {
    type Result = u32;
    fn name(
        &self,
        _req: QueryNameRequest,
    ) -> Result<edge_application::QueryNameResponse<'_>, QueryError> {
        Ok(edge_application::QueryNameResponse { name: "count" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> futures::future::BoxFuture<'_, Result<QueryResultResponse<u32>, QueryError>> {
        let v = self.0;
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

#[tokio::test]
async fn test_query_svc_facade_execute_returns_wrapped_value() {
    assert_eq!(
        Count(99).execute(QueryExecuteRequest).await.unwrap().result,
        99
    );
}

#[tokio::test]
async fn test_query_svc_facade_name_returns_identifier() {
    assert_eq!(Count(0).name(QueryNameRequest).unwrap().name, "count");
}
