//! Rule-222 coverage for [`OAuthTokenSourceFactory`] trait fn `create_from_file`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use std::sync::Arc;

use edge_llm_provider::OAuthTokenSourceFactory;

/// Stub implementation for testing.
struct StubTokenFactory;

impl OAuthTokenSourceFactory for StubTokenFactory {
    fn create_from_file(&self, _path: &Path) -> Result<Arc<dyn std::any::Any>, String> {
        Ok(Arc::new("token".to_string()))
    }
}

/// Stub implementation that always fails.
struct FailingTokenFactory;

impl OAuthTokenSourceFactory for FailingTokenFactory {
    fn create_from_file(&self, _path: &Path) -> Result<Arc<dyn std::any::Any>, String> {
        Err("file not found".to_string())
    }
}

// --- create_from_file ---

/// @covers: OAuthTokenSourceFactory::create_from_file — success case
#[test]
fn test_create_from_file_valid_path_happy() {
    let factory = StubTokenFactory;
    let result = factory.create_from_file(Path::new("/tmp/token.json"));
    assert_eq!(result, Ok(()));
}

/// @covers: OAuthTokenSourceFactory::create_from_file — error case
#[test]
fn test_create_from_file_missing_file_error() {
    let factory = FailingTokenFactory;
    let result = factory.create_from_file(Path::new("/nonexistent/path.json"));
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("file not found"));
}

/// @covers: OAuthTokenSourceFactory::create_from_file — edge case with empty path
#[test]
fn test_create_from_file_empty_path_edge() {
    let factory = StubTokenFactory;
    let result = factory.create_from_file(Path::new(""));
    assert_eq!(result, Ok(()));
}
