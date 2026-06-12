//! `QueryableRepository<T, Id>` — specification-based query extension to [`Repository`].

use futures::future::BoxFuture;

use super::Repository;
use crate::api::repository::types::Spec;
use crate::api::repository::RepositoryError;

/// Extension of [`Repository`] that supports specification-based queries.
///
/// Default implementations delegate to [`Repository::list`] and filter in Rust —
/// correct for in-memory backends.  Override in production backends to push
/// filtering down to the query engine (SQL WHERE, index scan, etc.).
///
/// ```rust,ignore
/// struct PaidOrders;
/// impl Spec<Order> for PaidOrders {
///     fn matches(&self, o: &Order) -> bool { o.status == Status::Paid }
/// }
///
/// let paid = repo.find_by(&PaidOrders).await?;
/// let count = repo.count_by(&PaidOrders).await?;
/// ```
pub trait QueryableRepository<T, Id>: Repository<T, Id>
where
    T: Send + Sync + 'static,
    Id: Send + Sync + 'static,
{
    /// Return all entities satisfying `spec`.
    fn find_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<Vec<T>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().filter(|e| spec.matches(e)).collect())
        })
    }

    /// Return the first entity satisfying `spec`, or `None`.
    fn find_one_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<Option<T>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().find(|e| spec.matches(e)))
        })
    }

    /// Count entities satisfying `spec`.
    fn count_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<usize, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().filter(|e| spec.matches(e)).count())
        })
    }
}
