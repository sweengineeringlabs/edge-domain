//! # edge-domain-observer
//!
//! **Observer contract layer** — the abstraction through which handlers observe their own behavior.
//!
//! This crate provides trait contracts for **request-scoped observability**: handlers call methods on
//! [`ObserverContext`] to emit traces, logs, and metrics without knowing or caring how that telemetry
//! is collected or where it goes.
//!
//! ## What it does
//!
//! From the **handler's perspective**, this IS the observer. Handlers see:
//! - [`HandlerTracer`] — emit distributed trace spans
//! - [`LogDrain`] — write structured log records
//! - [`MetricRegistry`] — increment counters, set gauges, record histograms
//!
//! All accessed via a single [`ObserverContext`] injected at request time.
//!
//! ## How it works
//!
//! 1. **In tests:** Use [`StdObserveFactory::noop_observer_context()`] — handlers observe, but nothing happens
//! 2. **In production:** Wire SDK-backed implementations (OpenTelemetry, Prometheus) to the same interface
//!
//! The handler code never changes. The observer backend is pluggable.
//!
//! ## Example
//!
//! ```ignore
//! // Handler receives observer at dispatch time
//! let observer = StdObserveFactory::noop_observer_context();
//!
//! // Handler observes its behavior
//! observer.tracer().start_span("my_handler", "execute").finish();
//! observer.metrics().counter("requests").increment(1);
//! observer.drain().emit(LogRecord::new("INFO", "handler", "started"));
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::Counter;
pub use saf::Gauge;
pub use saf::HandlerTracer;
pub use saf::Histogram;
pub use saf::LogDrain;
pub use saf::MetricRegistry;
pub use saf::NoopObserve;
pub use saf::ObserveBootstrap;
pub use saf::ObserverContext;
pub use saf::Span;
pub use saf::COUNTER_SVC;
pub use saf::GAUGE_SVC;
pub use saf::HANDLER_TRACER_SVC;
pub use saf::HISTOGRAM_SVC;
pub use saf::LOG_DRAIN_SVC;
pub use saf::METRIC_REGISTRY_SVC;
pub use saf::NOOP_OBSERVE_SVC;
pub use saf::OBSERVE_CONTEXT_SVC;
pub use saf::OBSERVE_FACTORY_SVC;
pub use saf::SPAN_SVC;
