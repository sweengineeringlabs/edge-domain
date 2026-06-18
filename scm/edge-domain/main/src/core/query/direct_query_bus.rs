//! `DirectQueryBus` — inline query dispatch with no queuing.

use futures::future::BoxFuture;

use crate::api::Query;
use crate::api::QueryBus;
use crate::api::QueryError;
use crate::api::QueryBusFactory;

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

    /// @covers: new
    #[test]
    fn test_new_constructs_bus() {
        let _bus = DirectQueryBus::<String>::new();
    }

    /// @covers: dispatch
    #[test]
    fn test_dispatch_returns_query_result() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let bus = DirectQueryBus::<String>::new();
        let result = rt
            .block_on(bus.dispatch(Box::new(DirectQueryBusEcho("pong".into()))))
            .unwrap();
        assert_eq!(result, "pong");
    }
}
