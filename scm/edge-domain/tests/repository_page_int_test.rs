//! Integration tests for `Repository` pagination — `list_page`, `exists`, `count`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{InMemoryRepository, Page, Repository};

/// @covers: list_page
#[tokio::test]
async fn test_repository_list_page_returns_correct_window() {
    let repo = InMemoryRepository::<String, u32>::new();
    for i in 0..10u32 {
        repo.save(i, format!("item-{i}")).await.unwrap();
    }
    let page: Page<String> = repo.list_page(0, 3).await.unwrap();
    assert_eq!(page.items.len(), 3);
    assert_eq!(page.total, 10);
    assert_eq!(page.offset, 0);
    assert_eq!(page.limit, 3);
    assert!(page.has_more());
    assert_eq!(page.next_offset(), Some(3));
}

/// @covers: list_page
#[tokio::test]
async fn test_repository_list_page_last_page_has_no_more() {
    let repo = InMemoryRepository::<String, u32>::new();
    for i in 0..5u32 {
        repo.save(i, format!("item-{i}")).await.unwrap();
    }
    let page: Page<String> = repo.list_page(3, 10).await.unwrap();
    assert_eq!(page.items.len(), 2);
    assert_eq!(page.total, 5);
    assert!(!page.has_more());
    assert_eq!(page.next_offset(), None);
}

/// @covers: list_page
#[tokio::test]
async fn test_repository_list_page_beyond_end_returns_empty() {
    let repo = InMemoryRepository::<String, u32>::new();
    repo.save(1u32, "a".into()).await.unwrap();
    let page: Page<String> = repo.list_page(10, 5).await.unwrap();
    assert!(page.items.is_empty());
    assert_eq!(page.total, 1);
    assert!(!page.has_more());
}

/// @covers: exists
#[tokio::test]
async fn test_repository_exists_returns_true_for_saved_entity() {
    let repo = InMemoryRepository::<String, u32>::new();
    repo.save(42u32, "hello".into()).await.unwrap();
    assert!(repo.exists(&42u32).await.unwrap());
}

/// @covers: exists
#[tokio::test]
async fn test_repository_exists_returns_false_for_missing_entity() {
    let repo = InMemoryRepository::<String, u32>::new();
    assert!(!repo.exists(&99u32).await.unwrap());
}

/// @covers: count
#[tokio::test]
async fn test_repository_count_returns_zero_when_empty() {
    let repo = InMemoryRepository::<String, u32>::new();
    assert_eq!(repo.count().await.unwrap(), 0);
}

/// @covers: count
#[tokio::test]
async fn test_repository_count_reflects_saved_entities() {
    let repo = InMemoryRepository::<String, u32>::new();
    repo.save(1u32, "a".into()).await.unwrap();
    repo.save(2u32, "b".into()).await.unwrap();
    assert_eq!(repo.count().await.unwrap(), 2);
}

/// @covers: count
#[tokio::test]
async fn test_repository_count_decrements_after_delete() {
    let repo = InMemoryRepository::<String, u32>::new();
    repo.save(1u32, "a".into()).await.unwrap();
    repo.save(2u32, "b".into()).await.unwrap();
    repo.delete(&1u32).await.unwrap();
    assert_eq!(repo.count().await.unwrap(), 1);
}
