//! [`CounterLookupRequest`] — request to resolve a named counter instrument.

/// Request for the [`Counter`](crate::api::Counter) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CounterLookupRequest {
    /// The instrument name.
    pub name: String,
}
