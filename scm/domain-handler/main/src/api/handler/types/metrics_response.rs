//! [`MetricsResponse`] — wrapper for the active `MetricRegistry`.

use crate::api::handler::traits::MetricRegistry;

/// Result of [`ObserverContext::metrics`](crate::api::handler::traits::ObserverContext::metrics).
pub struct MetricsResponse<'a> {
    /// The active metric registry.
    pub metrics: Box<dyn MetricRegistry + 'a>,
}
