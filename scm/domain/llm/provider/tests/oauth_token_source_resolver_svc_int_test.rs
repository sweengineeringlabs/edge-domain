//! SAF service tests — oauth-token-source-resolver contract identifier.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_provider::OAUTH_TOKEN_SOURCE_RESOLVER_SVC;

/// @covers: OAUTH_TOKEN_SOURCE_RESOLVER_SVC — equals the canonical string
#[test]
fn test_oauth_token_source_resolver_svc_equals_canonical_happy() {
    assert_eq!(
        OAUTH_TOKEN_SOURCE_RESOLVER_SVC,
        "oauth_token_source_resolver"
    );
}

/// @covers: OAUTH_TOKEN_SOURCE_RESOLVER_SVC — is non-empty
#[test]
fn test_oauth_token_source_resolver_svc_is_non_empty_error() {
    assert!(!OAUTH_TOKEN_SOURCE_RESOLVER_SVC.is_empty());
}

/// @covers: OAUTH_TOKEN_SOURCE_RESOLVER_SVC — contains only lowercase ASCII and underscores
#[test]
fn test_oauth_token_source_resolver_svc_is_valid_identifier_edge() {
    assert!(OAUTH_TOKEN_SOURCE_RESOLVER_SVC
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '_'));
}
