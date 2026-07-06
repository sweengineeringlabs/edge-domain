//! Integration tests for `QueryError`.

use edge_domain::QueryError;

/// @covers: QueryError
#[test]
fn test_query_error_invalid_input_display_contains_message() {
    let e = QueryError::InvalidInput("bad id format".into());
    assert!(e.to_string().contains("bad id format"));
}

/// @covers: QueryError
#[test]
fn test_query_error_not_found_display_contains_message() {
    let e = QueryError::NotFound("item-42".into());
    assert!(e.to_string().contains("item-42"));
}

/// @covers: QueryError
#[test]
fn test_query_error_internal_display_contains_message() {
    let e = QueryError::Internal("read failed".into());
    assert!(e.to_string().contains("read failed"));
}

/// @covers: QueryError
#[test]
fn test_query_error_has_no_rule_violation_variant() {
    // Queries are read-only — RuleViolation is a write concern only.
    // This test documents the intentional absence by exhausting the match:
    // if a RuleViolation variant were ever added, this would fail to compile.
    let e = QueryError::NotFound("x".into());
    let matched = match e {
        QueryError::InvalidInput(_) | QueryError::NotFound(_) | QueryError::Internal(_) => true,
    };
    assert!(
        matched,
        "QueryError should only ever contain the three known read-only variants"
    );
}
