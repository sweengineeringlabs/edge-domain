//! `QueryBus` trait — dispatches queries and returns their results.

use futures::future::BoxFuture;

use super::query::Query;
use super::query_error::QueryError;

/// Dispatches [`Query`] instances and returns their results.
///
/// The bus decouples the caller from the query implementation.
///
/// ```rust,ignore
/// impl QueryBus<Order> for DirectQueryBus<Order> {
///     fn dispatch(&self, query: Box<dyn Query<Order>>) -> BoxFuture<'_, Result<Order, QueryError>> {
///         Box::pin(async move { query.execute().await })
///     }
/// }
/// ```
pub trait QueryBus<R: Send + 'static>: Send + Sync {
    /// Dispatch a query and return its result.
    fn dispatch(&self, query: Box<dyn Query<R>>) -> BoxFuture<'_, Result<R, QueryError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_bus_is_object_safe() {
        fn _assert(_: &dyn QueryBus<String>) {}
    }

    struct NoopBus;
    impl QueryBus<String> for NoopBus {
        fn dispatch(
            &self,
            query: Box<dyn Query<String>>,
        ) -> BoxFuture<'_, Result<String, QueryError>> {
            Box::pin(async move { query.execute().await })
        }
    }

    struct EchoQuery(String);
    impl Query<String> for EchoQuery {
        fn name(&self) -> &str {
            "echo"
        }
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }

    #[tokio::test]
    async fn test_dispatch_returns_query_result() {
        let r = NoopBus
            .dispatch(Box::new(EchoQuery("ok".into())))
            .await
            .unwrap();
        assert_eq!(r, "ok");
    }
}
