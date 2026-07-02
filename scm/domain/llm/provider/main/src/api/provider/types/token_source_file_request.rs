use std::path::Path;

/// Request for [`OauthTokenSourceResolver::create_from_file`](crate::api::provider::traits::OauthTokenSourceResolver::create_from_file).
#[derive(Debug, Clone, Copy)]
pub struct TokenSourceFileRequest<'a> {
    /// Path to the credential file to initialize a token source from.
    pub path: &'a Path,
}
