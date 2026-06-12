#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — QueryableRepository is exported from the crate root.

use edge_domain::Domain;
use edge_domain::QueryableRepository;
use edge_domain::Spec;
use std::sync::Arc;

#[derive(Clone)]
struct Item {
    active: bool,
}

struct ActiveSpec;
impl Spec<Item> for ActiveSpec {
    fn matches(&self, item: &Item) -> bool {
        item.active
    }
}

#[tokio::test]
async fn test_queryable_repository_svc_facade_find_by_filters_correctly() {
    let repo: Arc<dyn QueryableRepository<Item, String>> =
        Domain::new_in_memory_queryable_repository();
    repo.save("a".into(), Item { active: true }).await.unwrap();
    repo.save("b".into(), Item { active: false }).await.unwrap();
    let found = repo.find_by(&ActiveSpec).await.unwrap();
    assert_eq!(found.len(), 1);
    assert!(found[0].active);
}

#[tokio::test]
async fn test_queryable_repository_svc_facade_count_by_returns_correct_count() {
    let repo: Arc<dyn QueryableRepository<Item, String>> =
        Domain::new_in_memory_queryable_repository();
    repo.save("x".into(), Item { active: true }).await.unwrap();
    repo.save("y".into(), Item { active: true }).await.unwrap();
    repo.save("z".into(), Item { active: false }).await.unwrap();
    let count = repo.count_by(&ActiveSpec).await.unwrap();
    assert_eq!(count, 2);
}
