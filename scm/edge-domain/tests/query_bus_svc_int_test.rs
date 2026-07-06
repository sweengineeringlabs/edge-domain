#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — QueryBus is exported from the crate root.

use edge_domain::Domain;
use edge_domain::Query;
use edge_domain::QueryBus;
use edge_domain::QueryDispatchRequest;
use edge_domain::QueryError;
use edge_domain::QueryExecuteRequest;
use edge_domain::QueryResultResponse;

struct FindById(u32);
impl Query for FindById {
    type Result = String;
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> futures::future::BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let id = self.0;
        Box::pin(async move { Ok(QueryResultResponse { result: format!("item-{}", id) }) })
    }
}

#[tokio::test]
async fn test_query_bus_svc_facade_dispatch_returns_result() {
    let bus = Domain::direct_query_bus::<String>();
    let result = bus.dispatch(QueryDispatchRequest { query: Box::new(FindById(7)) }).await.unwrap();
    assert_eq!(result.result, "item-7");
}
