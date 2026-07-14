//! `Histogram` — local decoupling boundary for a latency/distribution metric.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{HistogramRecordRequest, HistogramRecordResponse};

/// A metric that records value distributions (e.g. latency in milliseconds).
///
/// Declared locally so `api/` never references `edge_application_observer::Histogram`
/// directly in a type position (SEA `no_foreign_type`). Any real `Histogram`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait Histogram: Send + Sync {
    /// Record a single observation.
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, HandlerError>;
}
