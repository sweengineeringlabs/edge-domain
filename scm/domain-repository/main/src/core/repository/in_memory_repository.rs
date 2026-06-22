//! `Repository` and `QueryableRepository` impls for [`InMemoryRepository`].

use futures::future::BoxFuture;

use crate::api::InMemoryRepository;
use crate::api::RepositoryError;
use crate::api::{QueryableRepository, Repository};

impl<T, Id> Repository for InMemoryRepository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
    type Entity = T;
    type Id = Id;

    fn find<'a>(&'a self, id: &'a Id) -> BoxFuture<'a, Result<Option<T>, RepositoryError>> {
        let found = self.store.read().get(id).cloned();
        Box::pin(async move { Ok(found) })
    }

    fn save(&self, id: Id, entity: T) -> BoxFuture<'_, Result<(), RepositoryError>> {
        self.store.write().insert(id, entity);
        Box::pin(async move { Ok(()) })
    }

    fn delete<'a>(&'a self, id: &'a Id) -> BoxFuture<'a, Result<bool, RepositoryError>> {
        let removed = self.store.write().remove(id).is_some();
        Box::pin(async move { Ok(removed) })
    }

    fn list(&self) -> BoxFuture<'_, Result<Vec<T>, RepositoryError>> {
        let items: Vec<T> = self.store.read().values().cloned().collect();
        Box::pin(async move { Ok(items) })
    }
}

impl<T, Id> QueryableRepository for InMemoryRepository<T, Id>
where
    T: Clone + Send + Sync + 'static,
    Id: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Spec;
    use futures::executor::block_on;

    fn repo() -> InMemoryRepository<String, u32> {
        InMemoryRepository::new()
    }

    #[test]
    fn test_save_then_find_returns_entity_happy() {
        let r = repo();
        block_on(r.save(1, "hello".into())).unwrap_or_default();
        let found = block_on(r.find(&1)).unwrap_or(None);
        assert_eq!(found.as_deref(), Some("hello"));
    }

    #[test]
    fn test_find_missing_id_returns_none_error() {
        let r = repo();
        let found = block_on(r.find(&99)).unwrap_or(Some("x".into()));
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_existing_entity_returns_true_happy() {
        let r = repo();
        block_on(r.save(2, "bye".into())).unwrap_or_default();
        let removed = block_on(r.delete(&2)).unwrap_or(false);
        assert!(removed);
    }

    #[test]
    fn test_delete_missing_entity_returns_false_error() {
        let r = repo();
        let removed = block_on(r.delete(&42)).unwrap_or(true);
        assert!(!removed);
    }

    #[test]
    fn test_list_empty_repo_returns_empty_vec_edge() {
        let r = repo();
        let items = block_on(r.list()).unwrap_or_else(|_| vec!["x".into()]);
        assert!(items.is_empty());
    }

    #[test]
    fn test_find_by_spec_filters_correctly_happy() {
        struct InMemoryRepositoryStartsWithASpec;
        impl Spec<String> for InMemoryRepositoryStartsWithASpec {
            fn matches(&self, entity: &String) -> bool {
                entity.starts_with('a')
            }
        }
        let r = InMemoryRepository::new();
        block_on(r.save(1u32, "alpha".into())).unwrap_or_default();
        block_on(r.save(2u32, "beta".into())).unwrap_or_default();
        let results = block_on(r.find_by(&InMemoryRepositoryStartsWithASpec)).unwrap_or_default();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "alpha");
    }

    #[test]
    fn test_find_one_by_no_match_returns_none_error() {
        struct InMemoryRepositoryNeverMatchSpec;
        impl Spec<String> for InMemoryRepositoryNeverMatchSpec {}
        let r: InMemoryRepository<String, u32> = InMemoryRepository::new();
        let found =
            block_on(r.find_one_by(&InMemoryRepositoryNeverMatchSpec)).unwrap_or(Some("x".into()));
        assert!(found.is_none());
    }

    #[test]
    fn test_count_by_matches_correct_count_edge() {
        struct InMemoryRepositoryStartsWithASpec;
        impl Spec<String> for InMemoryRepositoryStartsWithASpec {
            fn matches(&self, entity: &String) -> bool {
                entity.starts_with('a')
            }
        }
        let r = InMemoryRepository::new();
        block_on(r.save(1u32, "ant".into())).unwrap_or_default();
        block_on(r.save(2u32, "bear".into())).unwrap_or_default();
        block_on(r.save(3u32, "ape".into())).unwrap_or_default();
        let n = block_on(r.count_by(&InMemoryRepositoryStartsWithASpec)).unwrap_or(0);
        assert_eq!(n, 2);
    }
}
