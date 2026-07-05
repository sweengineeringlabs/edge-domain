//! [`ClockBootstrap`] — constructor contract for clock implementations.

use std::time::SystemTime;

use crate::api::clock::errors::ClockError;
use crate::api::clock::types::{BootstrapNameRequest, BootstrapNameResponse, FixedClock, StdClockFactory, SystemClock};

/// Bootstrap trait for the two standard `Clock` implementations.
pub trait ClockBootstrap {
    /// Identifies this bootstrap implementation.
    fn bootstrap_name(
        &self,
        _req: BootstrapNameRequest,
    ) -> Result<BootstrapNameResponse, ClockError> {
        Ok(BootstrapNameResponse { name: "clock" })
    }

    /// Construct the wall-clock `SystemClock` implementation.
    fn system() -> SystemClock
    where
        Self: Sized,
    {
        SystemClock
    }

    /// Construct a `FixedClock` frozen at `at`.
    fn fixed(at: SystemTime) -> FixedClock
    where
        Self: Sized,
    {
        FixedClock::new(at)
    }

    /// Return the standard clock-factory instance.
    fn std_factory() -> StdClockFactory
    where
        Self: Sized,
    {
        StdClockFactory
    }
}
