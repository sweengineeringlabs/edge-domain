mod registry_factory_svc;
mod registry_svc;

pub use registry_factory_svc::{RegistryFactory, StdRegistryFactory, REGISTRY_FACTORY_SVC};
pub use registry_svc::{InMemoryRegistry, Registry, RegistryError, REGISTRY_SVC};
