//! `DirectQueryBus` — inline query dispatch with no queuing.

use futures::future::BoxFuture;

use crate::api::query::Query;
use crate::api::query::QueryBus;
use crate::api::query::QueryError;
use crate::api::query::QueryBusFactory;

// impl Query for NoopQuery (see noop_query.rs)

/// Dispatches queries by calling `query.execute()` directly in the same task.
pub(crate) struct DirectQueryBus<R>(std::marker::PhantomData<fn() -> R>);

impl<R> DirectQueryBus<R> {
    pub(crate) fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<R: Send + 'static> QueryBusFactory for DirectQueryBus<R> {}

// impl QueryBus for DirectQueryBus
impl<R: Send + 'static> QueryBus for DirectQueryBus<R> {
    type Result = R;

    fn dispatch(&self, query: Box<dyn Query<Result = R>>) -> BoxFuture<'_, Result<R, QueryError>> {
        Box::pin(async move { query.execute().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DirectQueryBusEcho(String);
    impl Query for DirectQueryBusEcho {
        type Result = String;
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
        let bus = DirectQueryBus::<String>::new();
        let result = bus
            .dispatch(Box::new(DirectQueryBusEcho("pong".into())))
            .await
            .unwrap();
        assert_eq!(result, "pong");
    }
}
