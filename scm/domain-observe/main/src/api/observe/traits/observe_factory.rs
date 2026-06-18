//! `ObserveFactory` — assembles the observability triplet.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::types::StdObserveFactory;

use super::{HandlerTracer, LogDrain, MetricRegistry};

/// Constructs the three observability primitives as a unit.
///
/// SDK-backed factories (OTel, Prometheus) override `build_*` methods.
/// The noop `StdObserveFactory` is suitable for local dev and unit tests.
pub trait ObserveFactory: Send + Sync {
    /// Return the standard (noop) observe factory instance.
    fn std_factory() -> StdObserveFactory {
        StdObserveFactory
    }

    /// Check whether the observability backend is reachable.
    ///
    /// Returns [`ObserveError`] when the backend is unavailable or not yet
    /// initialised. Always returns `Ok(())` for noop implementations.
    fn validate(&self) -> Result<(), ObserveError> {
        Ok(())
    }

    /// Build a [`HandlerTracer`].
    fn build_handler_tracer(&self) -> Box<dyn HandlerTracer>;

    /// Build a [`MetricRegistry`].
    fn build_metric_registry(&self) -> Box<dyn MetricRegistry>;

    /// Build a [`LogDrain`].
    fn build_log_drain(&self) -> Box<dyn LogDrain>;
}
