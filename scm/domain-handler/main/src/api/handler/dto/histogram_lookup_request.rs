//! [`HistogramLookupRequest`] — request to resolve a named histogram instrument.

/// Request to resolve a [`Histogram`](crate::api::handler::traits::Histogram) instrument by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistogramLookupRequest {
    /// The histogram's stable name.
    pub name: String,
}
