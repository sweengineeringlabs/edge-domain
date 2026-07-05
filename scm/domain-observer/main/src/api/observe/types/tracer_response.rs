//! [`TracerResponse`] — wrapper for the active `HandlerTracer`.

use crate::api::HandlerTracer;

/// Result of [`ObserverContext::tracer`](crate::api::ObserverContext::tracer).
pub struct TracerResponse<'a> {
    /// The active handler tracer.
    pub tracer: &'a dyn HandlerTracer,
}
