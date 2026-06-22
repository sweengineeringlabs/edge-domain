//! `QueryableRepository` — specification-based query extension for `Repository`.

use futures::future::BoxFuture;

use crate::api::repository::errors::RepositoryError;
use crate::api::repository::traits::Repository;
use crate::api::repository::types::Spec;

/// Extends [`Repository`] with specification-based query methods.
///
/// The `Self::Entity: Clone` supertrait bound is required because the default
/// implementations load the full entity list and clone matching entries.
/// Concrete implementations may override these with more efficient queries.
pub trait QueryableRepository: Repository
where
    Self::Entity: Clone + Send + Sync + 'static,
{
    /// Returns a stable, non-empty identifier for this queryable repository.
    fn bootstrap_name(&self) -> &'static str {
        "queryable_repository"
    }

    /// Returns all entities that satisfy the given specification.
    fn find_by<'a>(
        &'a self,
        spec: &'a dyn Spec<Self::Entity>,
    ) -> BoxFuture<'a, Result<Vec<Self::Entity>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().filter(|e| spec.matches(e)).collect())
        })
    }

    /// Returns the first entity that satisfies the given specification, or `None`.
    fn find_one_by<'a>(
        &'a self,
        spec: &'a dyn Spec<Self::Entity>,
    ) -> BoxFuture<'a, Result<Option<Self::Entity>, RepositoryError>> {
        Box::pin(async move {
            let all = self.list().await?;
            Ok(all.into_iter().find(|e| spec.matches(e)))
        })
    }

    /// Returns the count of entities that satisfy the given specification.
    fn count_by<'a>(
        &'a self,
        spec: &'a dyn Spec<Self::Entity>,
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

    impl Repository for VecRepo {
        type Entity = u32;
        type Id = usize;

        fn find<'a>(
            &'a self,
            id: &'a usize,
        ) -> BoxFuture<'a, Result<Option<u32>, RepositoryError>> {
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

    impl QueryableRepository for VecRepo {}

    struct EvenSpec;

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_returns_nonempty_string_happy() {
        let repo = VecRepo { items: vec![] };
        assert!(!repo.bootstrap_name().is_empty());
    }

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_is_deterministic_error() {
        let repo = VecRepo { items: vec![] };
        assert_eq!(repo.bootstrap_name(), repo.bootstrap_name());
    }

    /// @covers: bootstrap_name
    #[test]
    fn test_bootstrap_name_is_static_str_edge() {
        let repo = VecRepo { items: vec![] };
        let _name: &'static str = repo.bootstrap_name();
    }
    impl Spec<u32> for EvenSpec {
        fn matches(&self, entity: &u32) -> bool {
            entity.is_multiple_of(2)
        }
    }

    #[test]
    fn test_find_by_matching_spec_returns_filtered_results_happy() {
        let repo = VecRepo {
            items: vec![1, 2, 3, 4],
        };
        let results = block_on(repo.find_by(&EvenSpec)).unwrap_or_default();
        assert_eq!(results, vec![2, 4]);
    }

    #[test]
    fn test_find_one_by_matching_spec_returns_first_happy() {
        let repo = VecRepo {
            items: vec![1, 2, 3, 4],
        };
        let result = block_on(repo.find_one_by(&EvenSpec)).unwrap_or(None);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_count_by_no_match_returns_zero_edge() {
        let repo = VecRepo {
            items: vec![1, 3, 5],
        };
        let n = block_on(repo.count_by(&EvenSpec)).unwrap_or(1);
        assert_eq!(n, 0);
    }
}
