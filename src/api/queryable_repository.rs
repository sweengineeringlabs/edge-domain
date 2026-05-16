//! `QueryableRepository<T, Id>` — specification-based query extension to [`Repository`].

use async_trait::async_trait;

use crate::api::repository::Repository;
use crate::api::repository_error::RepositoryError;
use crate::api::spec::Spec;

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
#[async_trait]
pub trait QueryableRepository<T, Id>: Repository<T, Id>
where
    T: Send + Sync + 'static,
    Id: Send + Sync + 'static,
{
    /// Return all entities satisfying `spec`.
    async fn find_by(&self, spec: &dyn Spec<T>) -> Result<Vec<T>, RepositoryError> {
        let all = self.list().await?;
        Ok(all.into_iter().filter(|e| spec.matches(e)).collect())
    }

    /// Return the first entity satisfying `spec`, or `None`.
    async fn find_one_by(&self, spec: &dyn Spec<T>) -> Result<Option<T>, RepositoryError> {
        let all = self.list().await?;
        Ok(all.into_iter().find(|e| spec.matches(e)))
    }

    /// Count entities satisfying `spec`.
    async fn count_by(&self, spec: &dyn Spec<T>) -> Result<usize, RepositoryError> {
        let all = self.list().await?;
        Ok(all.into_iter().filter(|e| spec.matches(e)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queryable_repository_is_object_safe() {
        fn _assert<T, Id>(_: &dyn QueryableRepository<T, Id>)
        where
            T: Send + Sync + 'static,
            Id: Send + Sync + 'static,
        {
        }
    }
}
