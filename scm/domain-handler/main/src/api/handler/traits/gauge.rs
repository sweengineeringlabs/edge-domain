//! `Gauge` — local decoupling boundary for a point-in-time value metric.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{GaugeSetRequest, GaugeSetResponse};

/// A metric that records a current absolute value (e.g. queue depth).
///
/// Declared locally so `api/` never references `edge_application_observer::Gauge`
/// directly in a type position (SEA `no_foreign_type`). Any real `Gauge`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait Gauge: Send + Sync {
    /// Set the gauge to `value`.
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError>;
}
