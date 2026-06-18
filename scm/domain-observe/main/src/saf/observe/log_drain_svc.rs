use crate::core::observe::NoopLogDrain;
pub use crate::api::LogDrain;
pub use crate::api::LogRecord;
pub use crate::api::StdObserveFactory;

/// Service-registry key for [`LogDrain`].
pub const LOG_DRAIN_SVC: &str = "edge.observe.log_drain";

impl StdObserveFactory {
    /// Return a noop [`LogDrain`] — suitable for unit tests and local dev.
    pub fn noop_log_drain() -> Box<dyn LogDrain> {
        Box::new(NoopLogDrain::new())
    }
}
