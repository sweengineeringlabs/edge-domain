//! `FixedClock` — deterministic [`Clock`](crate::api::clock::traits::clock::Clock) for tests.

use std::time::SystemTime;

/// Returns a fixed [`SystemTime`] on every [`now()`](crate::api::clock::traits::clock::Clock::now) call.
pub struct FixedClock {
    pub(crate) instant: SystemTime,
}

impl FixedClock {
    /// Construct a clock frozen at `instant`.
    pub fn new(instant: SystemTime) -> Self {
        Self { instant }
    }
}
