//! `DirectQueryBus` — inline query dispatch with no queuing.

use futures::future::BoxFuture;

use crate::api::query::Query;
use crate::api::query::QueryBus;
use crate::api::query::QueryError;
use crate::api::query::QueryBusFactory;

// impl Query for NoopQuery (see noop_query.rs)

/// Dispatches queries by calling `query.execute()` directly in the same task.
pub(crate) struct DirectQueryBus;

impl QueryBusFactory for DirectQueryBus {}

// impl QueryBus for DirectQueryBus
impl<R: Send + 'static> QueryBus<R> for DirectQueryBus {
    fn dispatch(&self, query: Box<dyn Query<R>>) -> BoxFuture<'_, Result<R, QueryError>> {
        Box::pin(async move { query.execute().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DirectQueryBusEcho(String);
    impl Query<String> for DirectQueryBusEcho {
        fn name(&self) -> &str {
            "echo"
        }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_returns_query_result() {
        let bus = DirectQueryBus;
        let result = bus
            .dispatch(Box::new(DirectQueryBusEcho("pong".into())))
            .await
            .unwrap();
        assert_eq!(result, "pong");
    }
}
