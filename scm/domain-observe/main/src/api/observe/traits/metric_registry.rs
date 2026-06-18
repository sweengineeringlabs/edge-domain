//! `MetricRegistry` — metric instrument factory.

use super::{Counter, Gauge, Histogram};

/// Creates named metric instruments.
pub trait MetricRegistry: Send + Sync {
    /// Return a counter for `name`.
    fn counter(&self, name: &str) -> Box<dyn Counter>;

    /// Return a histogram for `name`.
    fn histogram(&self, name: &str) -> Box<dyn Histogram>;

    /// Return a gauge for `name`.
    fn gauge(&self, name: &str) -> Box<dyn Gauge>;
}
