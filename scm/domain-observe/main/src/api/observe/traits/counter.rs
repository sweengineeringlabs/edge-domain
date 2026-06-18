//! `Counter` — a monotonically increasing metric.

/// A monotonically increasing integer metric.
pub trait Counter: Send + Sync {
    /// Increment the counter by `delta`.
    fn increment(&self, delta: u64);
}
