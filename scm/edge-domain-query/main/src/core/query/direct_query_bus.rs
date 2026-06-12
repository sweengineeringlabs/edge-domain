//! `QueryBus` impl for [`DirectQueryBus`] — inline dispatch, no queuing.

use futures::future::BoxFuture;

use crate::api::query::traits::Query;
use crate::api::query::traits::QueryBus;
use crate::api::query::types::DirectQueryBus;
use crate::api::query::QueryError;

impl<R: Send + 'static> QueryBus<R> for DirectQueryBus {
    fn dispatch(&self, query: Box<dyn Query<R>>) -> BoxFuture<'_, Result<R, QueryError>> {
        Box::pin(async move { query.execute().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DirectQueryBusOk(String);
    impl Query<String> for DirectQueryBusOk {
        fn execute(&self) -> BoxFuture<'_, Result<String, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }

    #[test]
    fn test_dispatch_ok_query_returns_value() {
        let bus = DirectQueryBus;
        let result = futures::executor::block_on(
            bus.dispatch(Box::new(DirectQueryBusOk("pong".into())))
        );
        assert_eq!(result.unwrap(), "pong");
    }
}
