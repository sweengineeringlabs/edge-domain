//! [`DrainResponse`] — wrapper for the active `LogDrain`.

use crate::api::handler::traits::LogDrain;

/// Result of [`ObserverContext::drain`](crate::api::handler::traits::ObserverContext::drain).
pub struct DrainResponse<'a> {
    /// The active log drain.
    pub drain: Box<dyn LogDrain + 'a>,
}
