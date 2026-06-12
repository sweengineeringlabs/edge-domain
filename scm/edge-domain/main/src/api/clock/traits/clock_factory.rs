//! [`ClockFactory`] — constructor contract for clock implementations.

use std::time::SystemTime;

use crate::api::clock::types::fixed_clock::FixedClock;
use crate::api::clock::types::system_clock::SystemClock;

/// Factory trait for the two standard [`Clock`](crate::api::clock::traits::clock::Clock) implementations.
///
/// A unit type that implements this trait gains default factory constructors for
/// [`SystemClock`] (wall-clock) and [`FixedClock`] (deterministic, for tests).
pub trait ClockFactory {
    /// Construct the wall-clock [`SystemClock`] implementation.
    fn system() -> SystemClock {
        SystemClock
    }

    /// Construct a [`FixedClock`] frozen at `at`.
    fn fixed(at: SystemTime) -> FixedClock {
        FixedClock::new(at)
    }
}
