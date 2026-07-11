//! [`HistogramLookupResponse`] — wrapper for a resolved histogram instrument.

use crate::api::handler::traits::Histogram;

/// Result of [`MetricRegistry::histogram`](crate::api::handler::traits::MetricRegistry::histogram).
pub struct HistogramLookupResponse {
    /// The resolved histogram instrument.
    pub histogram: Box<dyn Histogram>,
}
