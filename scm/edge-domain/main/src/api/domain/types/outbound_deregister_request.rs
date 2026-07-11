//! [`OutboundDeregisterRequest`] — request to remove an outbound handle.

/// Request to remove the handle registered under `name`.
pub struct OutboundDeregisterRequest {
    /// The name of the handle to remove.
    pub name: String,
}
