//! `Query` impl for [`NoopQuery`] — no-op that always returns `Ok(())`.

use futures::future::BoxFuture;

use crate::api::query::traits::Query;
use crate::api::query::types::NoopQuery;
use crate::api::query::QueryError;

impl Query for NoopQuery {
    type Result = ();

    fn execute(&self) -> BoxFuture<'_, Result<(), QueryError>> {
        Box::pin(async { Ok(()) })
    }
}
