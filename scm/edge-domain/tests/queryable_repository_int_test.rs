//! Integration tests for `QueryableRepository` — spec-based queries.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Domain, QueryableRepository, RepositoryError, RepositorySaveRequest, Spec, SpecMatchesRequest,
    SpecMatchesResponse, SpecRequest,
};
use std::sync::Arc;

// ── Fixtures ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
struct Product {
    name: String,
    price_cents: u32,
}

struct ExpensiveProducts {
    threshold: u32,
}
impl Spec for ExpensiveProducts {
    type Entity = Product;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, Product>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: req.entity.price_cents >= self.threshold,
        })
    }
}

struct NameStartsWith(String);
impl Spec for NameStartsWith {
    type Entity = Product;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, Product>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: req.entity.name.starts_with(&self.0),
        })
    }
}

async fn seeded_repo() -> Arc<dyn QueryableRepository<Entity = Product, Id = u32>> {
    let repo = Domain.new_in_memory_queryable_repository::<Product, u32>();
    repo.save(RepositorySaveRequest {
        id: 1,
        entity: Product {
            name: "Apple".into(),
            price_cents: 150,
        },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: 2,
        entity: Product {
            name: "Avocado".into(),
            price_cents: 300,
        },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: 3,
        entity: Product {
            name: "Banana".into(),
            price_cents: 80,
        },
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: 4,
        entity: Product {
            name: "Blueberry".into(),
            price_cents: 500,
        },
    })
    .await
    .unwrap();
    repo
}

// ── find_by ───────────────────────────────────────────────────────────────────

/// @covers: find_by
#[tokio::test]
async fn test_queryable_repository_find_by_returns_matching_entities() {
    let repo = seeded_repo().await;
    let results = repo
        .find_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 200 }),
        })
        .await
        .unwrap()
        .items;
    assert_eq!(results.len(), 2); // Avocado (300), Blueberry (500)
    assert!(results.iter().all(|p| p.price_cents >= 200));
}

/// @covers: find_by
#[tokio::test]
async fn test_queryable_repository_find_by_returns_empty_when_none_match() {
    let repo = seeded_repo().await;
    let results = repo
        .find_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 9999 }),
        })
        .await
        .unwrap()
        .items;
    assert!(results.is_empty());
}

/// @covers: find_by
#[tokio::test]
async fn test_queryable_repository_find_by_name_prefix_filters_correctly() {
    let repo = seeded_repo().await;
    let results = repo
        .find_by(SpecRequest {
            spec: Box::new(NameStartsWith("A".into())),
        })
        .await
        .unwrap()
        .items;
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|p| p.name.starts_with('A')));
}

// ── find_one_by ───────────────────────────────────────────────────────────────

/// @covers: find_one_by
#[tokio::test]
async fn test_queryable_repository_find_one_by_returns_first_match() {
    let repo = seeded_repo().await;
    let result = repo
        .find_one_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 400 }),
        })
        .await
        .unwrap()
        .entity;
    assert!(result.is_some());
    assert!(result.unwrap().price_cents >= 400);
}

/// @covers: find_one_by
#[tokio::test]
async fn test_queryable_repository_find_one_by_returns_none_when_no_match() {
    let repo = seeded_repo().await;
    let result = repo
        .find_one_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 9999 }),
        })
        .await
        .unwrap()
        .entity;
    assert!(result.is_none());
}

// ── count_by ──────────────────────────────────────────────────────────────────

/// @covers: count_by
#[tokio::test]
async fn test_queryable_repository_count_by_returns_correct_count() {
    let repo = seeded_repo().await;
    let count = repo
        .count_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 200 }),
        })
        .await
        .unwrap()
        .count;
    assert_eq!(count, 2); // Avocado (300), Blueberry (500)
}

/// @covers: count_by
#[tokio::test]
async fn test_queryable_repository_count_by_returns_zero_when_no_match() {
    let repo = seeded_repo().await;
    let count = repo
        .count_by(SpecRequest {
            spec: Box::new(ExpensiveProducts { threshold: 9999 }),
        })
        .await
        .unwrap()
        .count;
    assert_eq!(count, 0);
}

/// @covers: count_by
#[tokio::test]
async fn test_queryable_repository_count_by_matches_find_by_len() {
    let repo = seeded_repo().await;
    let found = repo
        .find_by(SpecRequest {
            spec: Box::new(NameStartsWith("B".into())),
        })
        .await
        .unwrap()
        .items;
    let count = repo
        .count_by(SpecRequest {
            spec: Box::new(NameStartsWith("B".into())),
        })
        .await
        .unwrap()
        .count;
    assert_eq!(found.len(), count);
}
