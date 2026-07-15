//! [`GaugeSetResponse`] — wrapper for a successful gauge update.

/// Result of [`Gauge::set`](crate::api::handler::traits::Gauge::set).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GaugeSetResponse;
