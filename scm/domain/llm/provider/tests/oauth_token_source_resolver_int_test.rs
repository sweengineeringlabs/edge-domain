//! Rule-222 coverage for [`OauthTokenSourceResolver`] trait fn `create_from_file`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;

use edge_llm_provider::{
    OauthTokenSourceError, OauthTokenSourceResolver, TokenSourceFileRequest,
    TokenSourceInitResponse,
};

/// Test double for testing.
struct TokenFactoryDouble;

impl OauthTokenSourceResolver for TokenFactoryDouble {
    fn create_from_file(
        &self,
        _req: TokenSourceFileRequest<'_>,
    ) -> Result<TokenSourceInitResponse, OauthTokenSourceError> {
        Ok(TokenSourceInitResponse {
            source: Arc::new("token".to_string()),
        })
    }
}

/// Test double that always fails.
struct FailingTokenFactoryDouble;

impl OauthTokenSourceResolver for FailingTokenFactoryDouble {
    fn create_from_file(
        &self,
        _req: TokenSourceFileRequest<'_>,
    ) -> Result<TokenSourceInitResponse, OauthTokenSourceError> {
        Err(OauthTokenSourceError::CredentialFileUnreadable(
            "file not found".to_string(),
        ))
    }
}

// --- create_from_file ---

/// @covers: OauthTokenSourceResolver::create_from_file — success case
#[test]
fn test_create_from_file_valid_path_happy() {
    let resolver = TokenFactoryDouble;
    let path = Path::new("/tmp/token.json");
    let result = resolver.create_from_file(TokenSourceFileRequest { path });
    let response = result.expect("expected Ok response");
    let source = response
        .source
        .downcast_ref::<String>()
        .expect("expected String token source");
    assert_eq!(source, "token");
}

/// @covers: OauthTokenSourceResolver::create_from_file — error case
#[test]
fn test_create_from_file_missing_file_error() {
    let resolver = FailingTokenFactoryDouble;
    let path = Path::new("/nonexistent/path.json");
    let result = resolver.create_from_file(TokenSourceFileRequest { path });
    match result {
        Ok(_) => panic!("expected Err response for missing file, got Ok"),
        Err(err) => assert_eq!(
            err,
            OauthTokenSourceError::CredentialFileUnreadable("file not found".to_string())
        ),
    }
}

/// @covers: OauthTokenSourceResolver::create_from_file — edge case with empty path
#[test]
fn test_create_from_file_empty_path_edge() {
    let resolver = TokenFactoryDouble;
    let path = Path::new("");
    let result = resolver.create_from_file(TokenSourceFileRequest { path });
    let response = result.expect("expected Ok response even for empty path");
    let source = response
        .source
        .downcast_ref::<String>()
        .expect("expected String token source");
    assert_eq!(source, "token");
}
