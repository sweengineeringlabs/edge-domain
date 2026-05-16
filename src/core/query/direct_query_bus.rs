//! `DirectQueryBus` — inline query dispatch with no queuing.

use async_trait::async_trait;

use crate::api::query::Query;
use crate::api::query::QueryBus;
use crate::api::query::QueryError;

/// Dispatches queries by calling `query.execute()` directly in the same task.
pub(crate) struct DirectQueryBus;

#[async_trait]
impl<R: Send + 'static> QueryBus<R> for DirectQueryBus {
    async fn dispatch(&self, query: Box<dyn Query<R>>) -> Result<R, QueryError> {
        query.execute().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct DirectQueryBusEcho(String);
    #[async_trait]
    impl Query<String> for DirectQueryBusEcho {
        fn name(&self) -> &str {
            "echo"
        }
        async fn execute(&self) -> Result<String, QueryError> {
            Ok(self.0.clone())
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
