//! [`CounterLookupRequest`] — request to resolve a named counter instrument.

/// Request to resolve a [`Counter`](crate::api::handler::traits::Counter) instrument by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterLookupRequest {
    /// The counter's stable name.
    pub name: String,
}
