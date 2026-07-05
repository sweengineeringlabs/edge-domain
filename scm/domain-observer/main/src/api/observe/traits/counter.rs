//! `Counter` — a monotonically increasing metric.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::types::{IncrementRequest, IncrementResponse};

/// A monotonically increasing integer metric.
pub trait Counter: Send + Sync {
    /// Increment the counter by `delta`.
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, ObserveError>;
}
