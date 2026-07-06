use crate::api::types::CacheControl;

/// Response for [`SchemaValidator::cache_control`](crate::api::traits::SchemaValidator::cache_control).
pub struct SchemaCacheControlResponse {
    /// The cache-control hint to attach to messages this validator approves.
    pub cache: Box<CacheControl>,
}
