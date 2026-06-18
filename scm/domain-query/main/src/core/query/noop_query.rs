//! `Query` and `QueryBus` impls for noop types — NoopQuery always returns `Ok(())`;
//! NoopQueryBus always returns `Err(QueryError::NotFound)`.

use futures::future::BoxFuture;

use crate::api::Query;
use crate::api::QueryBus;
use crate::api::NoopQuery;
use crate::api::NoopQueryBus;
use crate::api::QueryError;

impl Query for NoopQuery {
    type Result = ();

    fn execute(&self) -> BoxFuture<'_, Result<(), QueryError>> {
        Box::pin(async { Ok(()) })
    }
}

impl<R: Send + 'static> QueryBus for NoopQueryBus<R> {
    type Result = R;

    fn dispatch(
        &self,
        _: Box<dyn Query<Result = R>>,
    ) -> BoxFuture<'_, Result<R, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("noop".into())) })
    }
}
