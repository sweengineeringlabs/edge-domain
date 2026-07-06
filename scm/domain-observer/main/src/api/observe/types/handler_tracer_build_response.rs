//! [`HandlerTracerBuildResponse`] — wrapper for a constructed `HandlerTracer`.

use crate::api::HandlerTracer;

/// Result of [`ObserveBootstrap::build_handler_tracer`](crate::api::ObserveBootstrap::build_handler_tracer).
pub struct HandlerTracerBuildResponse {
    /// The constructed handler tracer.
    pub tracer: Box<dyn HandlerTracer>,
}
