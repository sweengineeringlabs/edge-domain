//! [`GaugeLookupResponse`] — wrapper for a resolved gauge instrument.

use crate::api::handler::traits::Gauge;

/// Result of [`MetricRegistry::gauge`](crate::api::handler::traits::MetricRegistry::gauge).
pub struct GaugeLookupResponse {
    /// The resolved gauge instrument.
    pub gauge: Box<dyn Gauge>,
}
