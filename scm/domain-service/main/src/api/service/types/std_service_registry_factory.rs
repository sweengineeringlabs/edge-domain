//! `StdServiceRegistryFactory` — default concrete factory for service registry construction.

/// The default concrete factory for constructing [`ServiceRegistry`](super::ServiceRegistry) instances.
///
/// Provides static factory methods for creating registries, noop services, and related instances.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StdServiceRegistryFactory;
