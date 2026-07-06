//! [`MetricsResponse`] — wrapper for the active `MetricRegistry`.

use crate::api::MetricRegistry;

/// Result of [`ObserverContext::metrics`](crate::api::ObserverContext::metrics).
pub struct MetricsResponse<'a> {
    /// The active metric registry.
    pub metrics: &'a dyn MetricRegistry,
}
