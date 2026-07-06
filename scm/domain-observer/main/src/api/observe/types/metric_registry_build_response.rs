//! [`MetricRegistryBuildResponse`] — wrapper for a constructed `MetricRegistry`.

use crate::api::MetricRegistry;

/// Result of [`ObserveBootstrap::build_metric_registry`](crate::api::ObserveBootstrap::build_metric_registry).
pub struct MetricRegistryBuildResponse {
    /// The constructed metric registry.
    pub metric_registry: Box<dyn MetricRegistry>,
}
