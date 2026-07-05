pub use crate::api::HandlerTracer;
use crate::api::NoopObserve;
use crate::api::StdObserveFactory;

/// Service-registry key for [`HandlerTracer`].
pub const HANDLER_TRACER_SVC: &str = "edge.observe.handler_tracer";

impl StdObserveFactory {
    /// Return a noop [`HandlerTracer`] — suitable for unit tests and local dev.
    pub fn noop_handler_tracer() -> Box<dyn HandlerTracer> {
        <StdObserveFactory as NoopObserve>::build_noop_handler_tracer()
    }
}
