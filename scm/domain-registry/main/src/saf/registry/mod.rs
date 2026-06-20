mod registry_bootstrap_svc;
mod registry_svc;

pub use registry_bootstrap_svc::{RegistryBootstrap, StdRegistryFactory, REGISTRY_FACTORY_SVC};
pub use registry_svc::{InMemoryRegistry, Registry, RegistryError, REGISTRY_SVC};
