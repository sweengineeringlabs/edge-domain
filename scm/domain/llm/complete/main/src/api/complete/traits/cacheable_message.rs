//! `CacheableMessage` — Anthropic prompt-caching hint attachment contract.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{
    CacheControl, CacheControlRequest, CacheControlResponse, MarkEphemeralRequest,
};

/// Types that can carry Anthropic prompt-caching hints.
pub trait CacheableMessage: Sized {
    /// Attach the given cache control hint to this message.
    fn with_cache_control(
        self,
        req: CacheControlRequest,
    ) -> Result<CacheControlResponse<Self>, CompleteError>;

    /// Convenience: mark this message as ephemeral (short-lived cache entry).
    fn mark_ephemeral(
        self,
        _req: MarkEphemeralRequest,
    ) -> Result<CacheControlResponse<Self>, CompleteError> {
        self.with_cache_control(CacheControlRequest {
            cache: Box::new(CacheControl::ephemeral()),
        })
    }
}
