//! `Clock` — injectable time source.

use std::time::SystemTime;

/// Source of the current wall time.
///
/// Inject [`SystemClock`](crate::api::clock::types::system_clock::SystemClock)
/// in production and
/// [`FixedClock`](crate::api::clock::types::fixed_clock::FixedClock)
/// in tests to make time-dependent domain logic deterministic.
pub trait Clock: Send + Sync {
    /// Return the current instant.
    fn now(&self) -> SystemTime;
}
