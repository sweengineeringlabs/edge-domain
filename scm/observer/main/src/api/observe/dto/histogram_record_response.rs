//! [`HistogramRecordResponse`] — wrapper for a successful histogram observation.

/// Result of [`Histogram::record`](crate::api::Histogram::record).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HistogramRecordResponse;
