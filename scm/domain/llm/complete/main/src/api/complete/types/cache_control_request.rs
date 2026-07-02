use crate::api::complete::types::CacheControl;

/// Request for [`CacheableMessage::with_cache_control`](crate::api::complete::traits::CacheableMessage::with_cache_control).
#[derive(Debug, Clone)]
pub struct CacheControlRequest {
    /// Cache control hint to attach.
    pub cache: Box<CacheControl>,
}
