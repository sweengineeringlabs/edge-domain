//! `Histogram` — latency / distribution metric.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::dto::{HistogramRecordRequest, HistogramRecordResponse};

/// A metric that records value distributions (e.g. latency in milliseconds).
pub trait Histogram: Send + Sync {
    /// Record a single observation.
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, ObserveError>;
}
