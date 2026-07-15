//! [`HistogramRecordRequest`] — request to record a histogram observation.

/// Request to record a single observation on a [`Histogram`](crate::api::Histogram).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HistogramRecordRequest {
    /// The observed value.
    pub value: f64,
}
