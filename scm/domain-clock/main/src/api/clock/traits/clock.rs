//! `Clock` — injectable time source.

use std::time::{Duration, SystemTime};

use crate::api::clock::errors::ClockError;

/// Source of the current wall time.
///
/// Inject `SystemClock` in production and `FixedClock` in tests to make
/// time-dependent domain logic deterministic.
pub trait Clock: Send + Sync {
    /// Return the current instant.
    fn now(&self) -> SystemTime;

    /// Return the elapsed duration since the Unix epoch.
    ///
    /// Returns [`ClockError::BeforeEpoch`] when the clock reports a time
    /// earlier than the Unix epoch.
    fn elapsed_since_epoch(&self) -> Result<Duration, ClockError> {
        self.now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ClockError::BeforeEpoch)
    }
}
