use std::sync::Arc;

use crate::api::NoopObserve;
pub use crate::api::ObserveContext;
use crate::api::StdObserveFactory;

/// Service-registry key for [`ObserveContext`].
pub const OBSERVE_CONTEXT_SVC: &str = "edge.observe.context";

impl StdObserveFactory {
    /// Return a noop [`ObserveContext`] — suitable for unit tests and local dev.
    pub fn noop_observe_context() -> Box<dyn ObserveContext> {
        <StdObserveFactory as NoopObserve>::build_noop_observe_context()
    }

    /// Return a noop [`ObserveContext`] wrapped in `Arc` — for structs that store shared ownership.
    pub fn noop_arc_observe_context() -> Arc<dyn ObserveContext> {
        Arc::from(StdObserveFactory::noop_observe_context())
    }
}
