//! `QueryBus` impl for `LoggingQueryBus` — delegates to inner bus, logs via `tracing`.

use futures::future::BoxFuture;

use crate::api::query::traits::Query;
use crate::api::query::traits::QueryBus;
use crate::api::query::types::LoggingQueryBus;
use crate::api::query::QueryError;

impl<R: Send + 'static> QueryBus for LoggingQueryBus<R> {
    type Result = R;

    fn dispatch(
        &self,
        query: Box<dyn Query<Result = R>>,
    ) -> BoxFuture<'_, Result<R, QueryError>> {
        let name = query.name().to_owned();
        let inner = self.inner.clone();
        Box::pin(async move {
            let result = inner.dispatch(query).await;
            match &result {
                Ok(_) => tracing::debug!(query = %name, "query dispatched ok"),
                Err(e) => tracing::debug!(query = %name, error = %e, "query dispatch error"),
            }
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;

    use super::*;
    use crate::api::query::types::{DirectQueryBus, NoopQueryBus};

    struct LoggingQueryBusOk(String);
    impl Query for LoggingQueryBusOk {
        type Result = String;

        fn name(&self) -> &str {
            "ok-query"
        }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }

    struct LoggingQueryBusErr;
    impl Query for LoggingQueryBusErr {
        type Result = String;

        fn name(&self) -> &str {
            "err-query"
        }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            Box::pin(async { Err(QueryError::NotFound("none".into())) })
        }
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_ok_query_returns_value() {
        let bus = LoggingQueryBus::<String> { inner: Arc::new(DirectQueryBus::new()) };
        let result = bus.dispatch(Box::new(LoggingQueryBusOk("pong".into()))).await;
        assert_eq!(result.unwrap(), "pong");
    }

    /// @covers: dispatch
    #[tokio::test]
    async fn test_dispatch_error_query_propagates_err() {
        let bus = LoggingQueryBus::<String> { inner: Arc::new(NoopQueryBus::new()) };
        assert!(bus.dispatch(Box::new(LoggingQueryBusErr)).await.is_err());
    }
}
