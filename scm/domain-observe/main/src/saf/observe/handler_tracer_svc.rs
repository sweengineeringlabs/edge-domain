use crate::core::observe::NoopHandlerTracer;
pub use crate::api::HandlerTracer;
pub use crate::api::StdObserveFactory;

/// Service-registry key for [`HandlerTracer`].
pub const HANDLER_TRACER_SVC: &str = "edge.observe.handler_tracer";

impl StdObserveFactory {
    /// Return a noop [`HandlerTracer`] — suitable for unit tests and local dev.
    pub fn noop_handler_tracer() -> Box<dyn HandlerTracer> {
        Box::new(NoopHandlerTracer::new())
    }
}
