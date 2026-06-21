//! `ObserveContext` — composition handle bundling all observability primitives.

use super::HandlerTracer;
use super::LogDrain;
use super::MetricRegistry;

/// A composition handle that bundles tracing, logging, and metric primitives.
///
/// Inject this as a seam into handlers and factories so the caller controls
/// which observability backend is active. Use [`crate::StdObserveFactory::noop_observe_context`]
/// in tests; wire an SDK-backed implementation in production.
pub trait ObserveContext: Send + Sync {
    /// Return the active [`HandlerTracer`] for distributed tracing.
    fn tracer(&self) -> &dyn HandlerTracer;

    /// Return the active [`LogDrain`] for structured log emission.
    fn drain(&self) -> &dyn LogDrain;

    /// Return the active [`MetricRegistry`] for counter/gauge/histogram instruments.
    fn metrics(&self) -> &dyn MetricRegistry;
}
