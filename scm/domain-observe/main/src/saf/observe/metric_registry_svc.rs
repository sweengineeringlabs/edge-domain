use crate::api::NoopObserve;
pub use crate::api::MetricRegistry;
pub use crate::api::StdObserveFactory;

/// Service-registry key for [`MetricRegistry`].
pub const METRIC_REGISTRY_SVC: &str = "edge.observe.metric_registry";

impl StdObserveFactory {
    /// Return a noop [`MetricRegistry`] — suitable for unit tests and local dev.
    pub fn noop_metric_registry() -> Box<dyn MetricRegistry> {
        <StdObserveFactory as NoopObserve>::build_noop_metric_registry()
    }
}
