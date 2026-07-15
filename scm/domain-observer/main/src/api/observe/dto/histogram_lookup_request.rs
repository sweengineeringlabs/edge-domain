//! [`HistogramLookupRequest`] — request to resolve a named histogram instrument.

/// Request for the [`Histogram`](crate::api::Histogram) instrument named `name`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistogramLookupRequest {
    /// The instrument name.
    pub name: String,
}
