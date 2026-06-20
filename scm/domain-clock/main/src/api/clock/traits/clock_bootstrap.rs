//! [`ClockBootstrap`] — constructor contract for clock implementations.

use std::time::SystemTime;

use crate::api::clock::types::{FixedClock, StdClockFactory, SystemClock};

/// Bootstrap trait for the two standard `Clock` implementations.
pub trait ClockBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(&self) -> &'static str {
        "clock"
    }

    /// Construct the wall-clock `SystemClock` implementation.
    fn system() -> SystemClock where Self: Sized {
        SystemClock
    }

    /// Construct a `FixedClock` frozen at `at`.
    fn fixed(at: SystemTime) -> FixedClock where Self: Sized {
        FixedClock::new(at)
    }

    /// Return the standard clock-factory instance.
    fn std_factory() -> StdClockFactory where Self: Sized {
        StdClockFactory
    }
}
