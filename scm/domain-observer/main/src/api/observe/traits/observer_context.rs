//! `ObserverContext` — composition handle bundling all observability primitives.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::dto::{DrainRequest, DrainResponse, MetricsRequest, MetricsResponse, TracerRequest, TracerResponse};

/// A composition handle that bundles tracing, logging, and metric primitives.
///
/// Inject this as a seam into handlers and factories so the caller controls
/// which observability backend is active. Use [`crate::StdObserveFactory::noop_observer_context`]
/// in tests; wire an SDK-backed implementation in production.
pub trait ObserverContext: Send + Sync {
    /// Return the active [`HandlerTracer`](super::HandlerTracer) for distributed tracing.
    fn tracer(&self, req: TracerRequest) -> Result<TracerResponse<'_>, ObserveError>;

    /// Return the active [`LogDrain`](super::LogDrain) for structured log emission.
    fn drain(&self, req: DrainRequest) -> Result<DrainResponse<'_>, ObserveError>;

    /// Return the active [`MetricRegistry`](super::MetricRegistry) for counter/gauge/histogram instruments.
    fn metrics(&self, req: MetricsRequest) -> Result<MetricsResponse<'_>, ObserveError>;
}
