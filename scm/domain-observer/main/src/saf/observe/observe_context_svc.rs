use std::sync::Arc;

use crate::api::NoopObserve;
pub use crate::api::ObserverContext;
use crate::api::StdObserveFactory;

/// Service-registry key for [`ObserverContext`].
pub const OBSERVE_CONTEXT_SVC: &str = "edge.observe.context";

impl StdObserveFactory {
    /// Return a noop [`ObserverContext`] — suitable for unit tests and local dev.
    pub fn noop_observer_context() -> Box<dyn ObserverContext> {
        <StdObserveFactory as NoopObserve>::build_noop_observer_context()
    }

    /// Return a noop [`ObserverContext`] wrapped in `Arc` — for structs that store shared ownership.
    pub fn noop_arc_observe_context() -> Arc<dyn ObserverContext> {
        Arc::from(StdObserveFactory::noop_observer_context())
    }
}
