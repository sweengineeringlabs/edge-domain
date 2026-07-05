//! [`GaugeSetResponse`] — wrapper for a successful gauge update.

/// Result of [`Gauge::set`](crate::api::Gauge::set).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GaugeSetResponse;
