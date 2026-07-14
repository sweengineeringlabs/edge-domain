#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — QueryableRepository is exported from the crate root.

use edge_application::Domain;
use edge_application::QueryableRepository;
use edge_application::{
    RepositoryError, RepositorySaveRequest, Spec, SpecMatchesRequest, SpecMatchesResponse,
    SpecRequest,
};
use std::sync::Arc;

#[derive(Clone)]
struct Item {
    active: bool,
}

struct ActiveSpec;
impl Spec for ActiveSpec {
    type Entity = Item;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, Item>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: req.entity.active,
        })
    }
}

#[tokio::test]
async fn test_queryable_repository_svc_facade_find_by_filters_correctly() {
    let repo: Arc<dyn QueryableRepository<Entity = Item, Id = String>> =
        Domain.new_in_memory_queryable_repository();
    repo.save(RepositorySaveRequest {
        id: "a".into(),
        entity: Item { active: true },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: "b".into(),
        entity: Item { active: false },
    })
    .await
    .unwrap();
    let found = repo
        .find_by(SpecRequest {
            spec: Box::new(ActiveSpec),
        })
        .await
        .unwrap()
        .items;
    assert_eq!(found.len(), 1);
    assert!(found[0].active);
}

#[tokio::test]
async fn test_queryable_repository_svc_facade_count_by_returns_correct_count() {
    let repo: Arc<dyn QueryableRepository<Entity = Item, Id = String>> =
        Domain.new_in_memory_queryable_repository();
    repo.save(RepositorySaveRequest {
        id: "x".into(),
        entity: Item { active: true },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: "y".into(),
        entity: Item { active: true },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: "z".into(),
        entity: Item { active: false },
    })
    .await
    .unwrap();
    let count = repo
        .count_by(SpecRequest {
            spec: Box::new(ActiveSpec),
        })
        .await
        .unwrap()
        .count;
    assert_eq!(count, 2);
}
