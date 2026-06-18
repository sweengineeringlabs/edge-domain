pub use crate::api::ObserveError;
pub use crate::api::ObserveFactory;
pub use crate::api::StdObserveFactory;

/// Service-registry key for [`ObserveFactory`].
pub const OBSERVE_FACTORY_SVC: &str = "edge.observe.factory";

impl StdObserveFactory {
    /// Return the standard [`StdObserveFactory`] backed by noop primitives.
    ///
    /// Wire SDK-backed factories (OTel, Prometheus) at the assembler layer.
    pub fn create_factory() -> StdObserveFactory {
        StdObserveFactory
    }
}
