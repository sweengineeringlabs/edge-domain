//! # edge-domain-observe
//!
//! Observability port contracts for the domain layer.
//!
//! Provides injectable, SDK-free contracts for handler tracing, metric
//! emission, and structured log drain. Inject noop implementations in tests;
//! wire SDK-backed implementations (OTel, Prometheus) via `edge-observe` in
//! production.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::Counter;
pub use saf::COUNTER_SVC;
pub use saf::Gauge;
pub use saf::GAUGE_SVC;
pub use saf::HandlerTracer;
pub use saf::Histogram;
pub use saf::HISTOGRAM_SVC;
pub use saf::LogDrain;
pub use saf::LogRecord;
pub use saf::MetricRegistry;
pub use saf::NoopObserve;
pub use saf::NOOP_OBSERVE_SVC;
pub use saf::ObserveError;
pub use saf::ObserveFactory;
pub use saf::Span;
pub use saf::SPAN_SVC;
pub use saf::StdObserveFactory;
pub use saf::HANDLER_TRACER_SVC;
pub use saf::LOG_DRAIN_SVC;
pub use saf::METRIC_REGISTRY_SVC;
pub use saf::OBSERVE_FACTORY_SVC;
