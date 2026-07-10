//! `Counter` — local decoupling boundary for a monotonically increasing metric.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{IncrementRequest, IncrementResponse};

/// A monotonically increasing integer metric.
///
/// Declared locally so `api/` never references `edge_domain_observer::Counter`
/// directly in a type position (SEA `no_foreign_type`). Any real `Counter`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait Counter: Send + Sync {
    /// Increment the counter by `delta`.
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, HandlerError>;
}
