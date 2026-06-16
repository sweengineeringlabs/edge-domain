mod registry;

pub use registry::{
    InMemoryRegistry, Registry, RegistryError, RegistryFactory, StdRegistryFactory,
    REGISTRY_FACTORY_SVC, REGISTRY_SVC,
};
