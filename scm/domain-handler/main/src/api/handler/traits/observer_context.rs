//! `ObserverContext` — local decoupling boundary bundling all observability primitives.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    DrainRequest, DrainResponse, MetricsRequest, MetricsResponse, ObserverContextAdapter,
    TracerRequest, TracerResponse,
};

/// A composition handle that bundles tracing, logging, and metric primitives.
///
/// Declared locally so `api/` never references `edge_domain_observer::ObserverContext`
/// directly in a type position (SEA `no_foreign_type`). Any real `ObserverContext`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait ObserverContext: Send + Sync {
    /// Return the active [`HandlerTracer`](super::HandlerTracer) for distributed tracing.
    fn tracer(&self, req: TracerRequest) -> Result<TracerResponse<'_>, HandlerError>;

    /// Return the active [`LogDrain`](super::LogDrain) for structured log emission.
    fn drain(&self, req: DrainRequest) -> Result<DrainResponse<'_>, HandlerError>;

    /// Return the active [`MetricRegistry`](super::MetricRegistry) for counter/gauge/histogram instruments.
    fn metrics(&self, req: MetricsRequest) -> Result<MetricsResponse<'_>, HandlerError>;

    /// Wrap an already type-erased `&dyn ForeignObserverContext` reference so
    /// it can bridge into this trait via the blanket impl in `core/`.
    fn wrap<T: ?Sized>(inner: &T) -> ObserverContextAdapter<'_, T>
    where
        Self: Sized,
    {
        ObserverContextAdapter(inner)
    }
}
