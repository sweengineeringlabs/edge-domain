//! [`OutboundGetResponse`] — wrapper for an outbound handle lookup result.

/// Result of [`OutboundRegistry::get`](crate::api::domain::traits::OutboundRegistry::get).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundGetResponse<H> {
    /// The handle, if one was registered under the requested name.
    pub handle: Option<H>,
}
