//! [`CounterLookupResponse`] — wrapper for a resolved counter instrument.

use crate::api::handler::traits::Counter;

/// Result of [`MetricRegistry::counter`](crate::api::handler::traits::MetricRegistry::counter).
pub struct CounterLookupResponse {
    /// The resolved counter instrument.
    pub counter: Box<dyn Counter>,
}
