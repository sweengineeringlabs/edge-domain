//! [`IncrementResponse`] — wrapper for a successful counter increment.

/// Result of [`Counter::increment`](crate::api::Counter::increment).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IncrementResponse;
