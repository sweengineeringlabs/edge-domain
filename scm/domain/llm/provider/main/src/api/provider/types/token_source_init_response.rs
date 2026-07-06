use std::any::Any;
use std::sync::Arc;

/// Response for [`OauthTokenSourceResolver::create_from_file`](crate::api::provider::traits::OauthTokenSourceResolver::create_from_file).
#[derive(Clone)]
pub struct TokenSourceInitResponse {
    /// Plugin-specific token source, opaque to the caller.
    pub source: Arc<dyn Any>,
}
