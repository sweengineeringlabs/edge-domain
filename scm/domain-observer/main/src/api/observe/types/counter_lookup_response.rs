//! [`CounterLookupResponse`] — wrapper for a resolved counter instrument.

use crate::api::Counter;

/// Result of [`MetricRegistry::counter`](crate::api::MetricRegistry::counter).
pub struct CounterLookupResponse {
    /// The resolved counter instrument.
    pub counter: Box<dyn Counter>,
}
