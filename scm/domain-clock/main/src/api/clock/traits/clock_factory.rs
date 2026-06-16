//! [`ClockFactory`] — constructor contract for clock implementations.

use std::time::SystemTime;

use crate::api::clock::types::{FixedClock, StdClockFactory, SystemClock};

/// Factory trait for the two standard `Clock` implementations.
pub trait ClockFactory {
    /// Construct the wall-clock `SystemClock` implementation.
    fn system() -> SystemClock {
        SystemClock
    }

    /// Construct a `FixedClock` frozen at `at`.
    fn fixed(at: SystemTime) -> FixedClock {
        FixedClock::new(at)
    }

    /// Return the standard clock-factory instance.
    fn std_factory() -> StdClockFactory {
        StdClockFactory
    }
}
