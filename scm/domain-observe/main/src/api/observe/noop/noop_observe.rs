//! `NoopObserve` — factory trait for all noop observability primitives.

use super::super::Counter;
use super::super::Gauge;
use super::super::HandlerTracer;
use super::super::Histogram;
use super::super::LogDrain;
use super::super::MetricRegistry;
use super::super::Span;

/// Factory trait that builds all noop observability primitives from a single type.
///
/// Implement this on a factory type (e.g. `StdObserveFactory`) to expose
/// allocation-free noop backends suitable for unit tests and local development.
/// Each method is an associated function — no instance required.
pub trait NoopObserve {
    /// Return a noop [`Counter`] that discards every increment.
    fn build_noop_counter() -> Box<dyn Counter>
    where
        Self: Sized;

    /// Return a noop [`Gauge`] that discards every set.
    fn build_noop_gauge() -> Box<dyn Gauge>
    where
        Self: Sized;

    /// Return a noop [`Histogram`] that discards every record.
    fn build_noop_histogram() -> Box<dyn Histogram>
    where
        Self: Sized;

    /// Return a noop [`Span`] that completes without recording.
    fn build_noop_span() -> Box<dyn Span>
    where
        Self: Sized;

    /// Return a noop [`HandlerTracer`] that produces silent spans.
    fn build_noop_handler_tracer() -> Box<dyn HandlerTracer>
    where
        Self: Sized;

    /// Return a noop [`LogDrain`] that discards every record.
    fn build_noop_log_drain() -> Box<dyn LogDrain>
    where
        Self: Sized;

    /// Return a noop [`MetricRegistry`] that discards all instruments.
    fn build_noop_metric_registry() -> Box<dyn MetricRegistry>
    where
        Self: Sized;
}
