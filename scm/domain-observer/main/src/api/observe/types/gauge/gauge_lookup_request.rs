//! [`GaugeLookupRequest`] — request to resolve a named gauge instrument.

/// Request for the [`Gauge`](crate::api::Gauge) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaugeLookupRequest {
    /// The instrument name.
    pub name: String,
}
