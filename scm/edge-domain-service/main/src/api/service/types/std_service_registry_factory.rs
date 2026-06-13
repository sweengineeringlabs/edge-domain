//! `StdServiceRegistryFactory` — concrete factory implementation.
//!
//! This type is kept in `api/types/` to satisfy the structural audit's one-type-per-file
//! rule. Its [`ServiceRegistryFactory`](crate::api::service::traits::service_registry_factory::ServiceRegistryFactory)
//! implementation lives in `core/service/service_registry_factory.rs`.

/// The default concrete factory for constructing [`ServiceRegistry`](super::ServiceRegistry) instances.
///
/// Implements [`ServiceRegistryFactory`](crate::api::service::traits::service_registry_factory::ServiceRegistryFactory).
#[derive(Debug, Default, Clone, Copy)]
pub struct StdServiceRegistryFactory;
