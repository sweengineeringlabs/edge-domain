pub use crate::api::ObserveBootstrap;
use crate::api::StdObserveFactory;

/// Service-registry key for [`ObserveBootstrap`].
pub const OBSERVE_FACTORY_SVC: &str = "edge.observe.factory";

impl StdObserveFactory {
    /// Return the standard [`StdObserveFactory`] backed by noop primitives.
    ///
    /// Wire SDK-backed factories (OTel, Prometheus) at the assembler layer.
    pub fn create_factory() -> StdObserveFactory {
        StdObserveFactory
    }
}
