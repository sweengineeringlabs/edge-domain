//! [`HistogramLookupResponse`] — wrapper for a resolved histogram instrument.

use crate::api::Histogram;

/// Result of [`MetricRegistry::histogram`](crate::api::MetricRegistry::histogram).
pub struct HistogramLookupResponse {
    /// The resolved histogram instrument.
    pub histogram: Box<dyn Histogram>,
}
