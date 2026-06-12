//! Integration tests — `SecurityError` variants and display.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_security::SecurityError;

/// @covers: SecurityError — display message for Unauthenticated
#[test]
fn test_unauthenticated_display_contains_not_authenticated_happy() {
    let msg = SecurityError::Unauthenticated.to_string();
    assert!(msg.contains("not authenticated"), "got: {msg}");
}

/// @covers: SecurityError — display message for MissingClaims
#[test]
fn test_missing_claims_display_contains_claim_happy() {
    let msg = SecurityError::MissingClaims.to_string();
    assert!(msg.contains("claim"), "got: {msg}");
}

/// @covers: SecurityError — display message for EmptyPrincipalId
#[test]
fn test_empty_principal_id_display_contains_empty_happy() {
    let msg = SecurityError::EmptyPrincipalId.to_string();
    assert!(msg.contains("empty"), "got: {msg}");
}

/// @covers: SecurityError — inequality between distinct variants
#[test]
fn test_variants_are_not_equal_error() {
    assert_ne!(SecurityError::Unauthenticated, SecurityError::MissingClaims);
    assert_ne!(SecurityError::Unauthenticated, SecurityError::EmptyPrincipalId);
    assert_ne!(SecurityError::MissingClaims, SecurityError::EmptyPrincipalId);
}

/// @covers: SecurityError — clone preserves variant equality
#[test]
fn test_clone_preserves_equality_edge() {
    assert_eq!(SecurityError::Unauthenticated.clone(), SecurityError::Unauthenticated);
    assert_eq!(SecurityError::MissingClaims.clone(), SecurityError::MissingClaims);
    assert_eq!(SecurityError::EmptyPrincipalId.clone(), SecurityError::EmptyPrincipalId);
}
