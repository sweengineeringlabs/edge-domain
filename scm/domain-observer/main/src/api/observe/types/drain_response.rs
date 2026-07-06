//! [`DrainResponse`] — wrapper for the active `LogDrain`.

use crate::api::LogDrain;

/// Result of [`ObserverContext::drain`](crate::api::ObserverContext::drain).
pub struct DrainResponse<'a> {
    /// The active log drain.
    pub drain: &'a dyn LogDrain,
}
