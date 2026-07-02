/// Response for [`CacheableMessage::with_cache_control`](crate::api::complete::traits::CacheableMessage::with_cache_control).
#[derive(Debug, Clone)]
pub struct CacheControlResponse<T> {
    /// The message with the cache control hint attached.
    pub message: T,
}
