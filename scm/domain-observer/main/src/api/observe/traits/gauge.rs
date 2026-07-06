//! `Gauge` — a point-in-time value metric.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::types::{GaugeSetRequest, GaugeSetResponse};

/// A metric that records a current absolute value (e.g. queue depth).
pub trait Gauge: Send + Sync {
    /// Set the gauge to `value`.
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, ObserveError>;
}
