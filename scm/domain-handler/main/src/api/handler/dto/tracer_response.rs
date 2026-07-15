//! [`TracerResponse`] — wrapper for the active `HandlerTracer`.

use crate::api::handler::traits::HandlerTracer;

/// Result of [`ObserverContext::tracer`](crate::api::handler::traits::ObserverContext::tracer).
pub struct TracerResponse<'a> {
    /// The active handler tracer.
    pub tracer: Box<dyn HandlerTracer + 'a>,
}
