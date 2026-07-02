//! Layer-level e2e coverage for the `OauthTokenSourceResolver` trait via a test-double implementer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;

use edge_llm_provider::{
    OauthTokenSourceError, OauthTokenSourceResolver, TokenSourceFileRequest,
    TokenSourceInitResponse,
};

struct ResolverDouble {
    should_fail: bool,
}

impl OauthTokenSourceResolver for ResolverDouble {
    fn create_from_file(
        &self,
        req: TokenSourceFileRequest<'_>,
    ) -> Result<TokenSourceInitResponse, OauthTokenSourceError> {
        if self.should_fail {
            return Err(OauthTokenSourceError::CredentialFileUnreadable(
                req.path.display().to_string(),
            ));
        }
        Ok(TokenSourceInitResponse {
            source: Arc::new(req.path.display().to_string()),
        })
    }
}

/// @covers: OauthTokenSourceResolver::create_from_file — succeeds for a readable path
#[test]
fn test_create_from_file_readable_path_happy() {
    let resolver = ResolverDouble { should_fail: false };
    let result = resolver.create_from_file(TokenSourceFileRequest {
        path: Path::new("/tmp/creds.json"),
    });
    let source = result.expect("resolver should succeed").source;
    assert_eq!(
        source.downcast_ref::<String>().expect("string source"),
        "/tmp/creds.json"
    );
}

/// @covers: OauthTokenSourceResolver::create_from_file — returns a typed error on failure
#[test]
fn test_create_from_file_unreadable_path_error() {
    let resolver = ResolverDouble { should_fail: true };
    let result = resolver.create_from_file(TokenSourceFileRequest {
        path: Path::new("/missing/creds.json"),
    });
    assert!(matches!(
        result,
        Err(OauthTokenSourceError::CredentialFileUnreadable(_))
    ));
}

/// @covers: OauthTokenSourceResolver::create_from_file — empty path is still a valid request shape
#[test]
fn test_create_from_file_empty_path_edge() {
    let resolver = ResolverDouble { should_fail: false };
    let result = resolver.create_from_file(TokenSourceFileRequest {
        path: Path::new(""),
    });
    let source = result.expect("resolver should succeed").source;
    assert_eq!(source.downcast_ref::<String>().expect("string source"), "");
}
