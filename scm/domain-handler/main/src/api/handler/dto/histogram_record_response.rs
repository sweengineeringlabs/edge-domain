//! [`HistogramRecordResponse`] — wrapper for a successful histogram observation.

/// Result of [`Histogram::record`](crate::api::handler::traits::Histogram::record).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HistogramRecordResponse;
