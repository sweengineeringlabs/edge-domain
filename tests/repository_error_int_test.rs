//! Integration tests for `RepositoryError`.

use edge_domain::RepositoryError;

/// @covers: RepositoryError
#[test]
fn test_repository_error_not_found_display_contains_message() {
    let e = RepositoryError::NotFound("entity-1".into());
    assert!(e.to_string().contains("entity-1"));
}

/// @covers: RepositoryError
#[test]
fn test_repository_error_conflict_display_contains_message() {
    let e = RepositoryError::Conflict("duplicate key".into());
    assert!(e.to_string().contains("duplicate key"));
}

/// @covers: RepositoryError
#[test]
fn test_repository_error_unavailable_display_contains_message() {
    let e = RepositoryError::Unavailable("connection refused".into());
    assert!(e.to_string().contains("connection refused"));
}

/// @covers: RepositoryError
#[test]
fn test_repository_error_internal_display_contains_message() {
    let e = RepositoryError::Internal("disk full".into());
    assert!(e.to_string().contains("disk full"));
}

/// @covers: RepositoryError
#[test]
fn test_repository_error_variants_are_distinct() {
    assert!(matches!(RepositoryError::NotFound("x".into()), RepositoryError::NotFound(_)));
    assert!(matches!(RepositoryError::Conflict("x".into()), RepositoryError::Conflict(_)));
    assert!(matches!(RepositoryError::Unavailable("x".into()), RepositoryError::Unavailable(_)));
    assert!(matches!(RepositoryError::Internal("x".into()), RepositoryError::Internal(_)));
}
