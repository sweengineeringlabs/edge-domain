//! [`GaugeLookupResponse`] — wrapper for a resolved gauge instrument.

use crate::api::Gauge;

/// Result of [`MetricRegistry::gauge`](crate::api::MetricRegistry::gauge).
pub struct GaugeLookupResponse {
    /// The resolved gauge instrument.
    pub gauge: Box<dyn Gauge>,
}
