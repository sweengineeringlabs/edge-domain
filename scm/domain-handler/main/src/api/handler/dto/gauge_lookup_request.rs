//! [`GaugeLookupRequest`] — request to resolve a named gauge instrument.

/// Request to resolve a [`Gauge`](crate::api::handler::traits::Gauge) instrument by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaugeLookupRequest {
    /// The gauge's stable name.
    pub name: String,
}
