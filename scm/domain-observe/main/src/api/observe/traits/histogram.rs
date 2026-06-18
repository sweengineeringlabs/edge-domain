//! `Histogram` — latency / distribution metric.

/// A metric that records value distributions (e.g. latency in milliseconds).
pub trait Histogram: Send + Sync {
    /// Record a single observation.
    fn record(&self, value: f64);
}
