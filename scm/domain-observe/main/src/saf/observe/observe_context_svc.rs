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
}
