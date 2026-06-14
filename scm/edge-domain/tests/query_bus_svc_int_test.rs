#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — QueryBus is exported from the crate root.

use edge_domain::Domain;
use edge_domain::Query;
use edge_domain::QueryBus;
use edge_domain::QueryError;

struct FindById(u32);
impl Query for FindById {
    type Result = String;
    fn name(&self) -> &str {
        "find-by-id"
    }
    fn execute(&self) -> futures::future::BoxFuture<'_, Result<String, QueryError>> {
        let id = self.0;
        Box::pin(async move { Ok(format!("item-{}", id)) })
    }
}

#[tokio::test]
async fn test_query_bus_svc_facade_dispatch_returns_result() {
    let bus = Domain::direct_query_bus::<String>();
    let result = bus.dispatch(Box::new(FindById(7))).await.unwrap();
    assert_eq!(result, "item-7");
}
