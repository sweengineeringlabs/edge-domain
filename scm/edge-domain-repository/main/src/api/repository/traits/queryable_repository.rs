//! `QueryableRepository` — specification-based query extension for `Repository`.

use futures::future::BoxFuture;

use crate::api::repository::errors::RepositoryError;
use crate::api::repository::traits::Repository;
use crate::api::repository::types::Spec;

/// Extends [`Repository`] with specification-based query methods.
///
/// Default implementations load the full list and filter in-process.
/// Concrete implementations may override these with more efficient queries.
pub trait QueryableRepository<T, Id>: Repository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: Send + Sync + 'static,
{
    /// Returns all entities that satisfy the given specification.
    fn find_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<Vec<T>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().filter(|e| spec.matches(e)).collect())
        })
    }

    /// Returns the first entity that satisfies the given specification, or `None`.
    fn find_one_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<Option<T>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().find(|e| spec.matches(e)))
        })
    }

    /// Returns the count of entities that satisfy the given specification.
    fn count_by<'a>(
        &'a self,
        spec: &'a dyn Spec<T>,
    ) -> BoxFuture<'a, Result<usize, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.iter().filter(|e| spec.matches(e)).count())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    struct VecRepo {
        items: Vec<u32>,
    }

    impl Repository<u32, usize> for VecRepo {
        fn find<'a>(&'a self, id: &'a usize) -> BoxFuture<'a, Result<Option<u32>, RepositoryError>> {
            let val = self.items.get(*id).copied();
            Box::pin(async move { Ok(val) })
        }
        fn save(&self, _id: usize, _entity: u32) -> BoxFuture<'_, Result<(), RepositoryError>> {
            Box::pin(async move { Ok(()) })
        }
        fn delete<'a>(&'a self, _id: &'a usize) -> BoxFuture<'a, Result<bool, RepositoryError>> {
            Box::pin(async move { Ok(false) })
        }
        fn list(&self) -> BoxFuture<'_, Result<Vec<u32>, RepositoryError>> {
            let vals = self.items.clone();
            Box::pin(async move { Ok(vals) })
        }
    }

    impl QueryableRepository<u32, usize> for VecRepo {}

    struct EvenSpec;
    impl Spec<u32> for EvenSpec {
        fn matches(&self, entity: &u32) -> bool {
            entity % 2 == 0
        }
    }

    #[test]
    fn test_find_by_matching_spec_returns_filtered_results_happy() {
        let repo = VecRepo { items: vec![1, 2, 3, 4] };
        let results = block_on(repo.find_by(&EvenSpec)).unwrap_or_default();
        assert_eq!(results, vec![2, 4]);
    }

    #[test]
    fn test_find_one_by_matching_spec_returns_first_happy() {
        let repo = VecRepo { items: vec![1, 2, 3, 4] };
        let result = block_on(repo.find_one_by(&EvenSpec)).unwrap_or(None);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_count_by_no_match_returns_zero_edge() {
        let repo = VecRepo { items: vec![1, 3, 5] };
        let n = block_on(repo.count_by(&EvenSpec)).unwrap_or(1);
        assert_eq!(n, 0);
    }
}
