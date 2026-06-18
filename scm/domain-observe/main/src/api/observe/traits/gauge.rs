//! `Gauge` — a point-in-time value metric.

/// A metric that records a current absolute value (e.g. queue depth).
pub trait Gauge: Send + Sync {
    /// Set the gauge to `value`.
    fn set(&self, value: f64);
}
