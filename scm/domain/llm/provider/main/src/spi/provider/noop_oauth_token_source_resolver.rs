//! `OauthTokenSourceResolver` no-op reference implementation.

use std::sync::Arc;

use crate::api::{
    OauthTokenSourceError, OauthTokenSourceResolver, TokenSourceFileRequest,
    TokenSourceInitResponse,
};

/// Reference resolver with no backing plugin — always returns a unit token source.
pub(crate) struct NoopOauthTokenSourceResolver;

impl OauthTokenSourceResolver for NoopOauthTokenSourceResolver {
    fn create_from_file(
        &self,
        _req: TokenSourceFileRequest<'_>,
    ) -> Result<TokenSourceInitResponse, OauthTokenSourceError> {
        Ok(TokenSourceInitResponse {
            source: Arc::new(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// @covers: create_from_file
    #[test]
    fn test_create_from_file_always_succeeds() {
        let resolver = NoopOauthTokenSourceResolver;
        let result = resolver.create_from_file(TokenSourceFileRequest {
            path: Path::new("/tmp/creds.json"),
        });
        let source = result.expect("noop resolver never fails").source;
        assert!(source.downcast_ref::<()>().is_some());
    }
}
