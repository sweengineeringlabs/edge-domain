//! [`OutboundGetRequest`] — request to look up an outbound handle.

/// Request to look up the handle registered under `name`.
pub struct OutboundGetRequest {
    /// The name to look up.
    pub name: String,
}
