pub use crate::api::LogDrain;
use crate::api::NoopObserve;
use crate::api::StdObserveFactory;

/// Service-registry key for [`LogDrain`].
pub const LOG_DRAIN_SVC: &str = "edge.observe.log_drain";

impl StdObserveFactory {
    /// Return a noop [`LogDrain`] — suitable for unit tests and local dev.
    pub fn noop_log_drain() -> Box<dyn LogDrain> {
        <StdObserveFactory as NoopObserve>::build_noop_log_drain()
    }
}
